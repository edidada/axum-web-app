use rbatis::RBatis;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::jwt::Claims;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub jti: String,
    pub username: String,
    pub role: String,
    pub csrf_hash: String,
    pub issued_at: i64,
    pub expires_at: i64,
    pub revoked_at: Option<i64>,
}

pub async fn init_store(rb: &RBatis) -> Result<(), String> {
    rb.exec(
        r#"CREATE TABLE IF NOT EXISTS auth_sessions (
            jti VARCHAR(64) PRIMARY KEY,
            username VARCHAR(255) NOT NULL,
            role VARCHAR(32) NOT NULL,
            csrf_hash VARCHAR(64) NOT NULL,
            issued_at BIGINT NOT NULL,
            expires_at BIGINT NOT NULL,
            revoked_at BIGINT
        )"#,
        vec![],
    )
    .await
    .map_err(|error| format!("create auth_sessions failed: {error}"))?;
    Ok(())
}

pub fn csrf_hash(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn verify_csrf(token: &str, expected_hash: &str) -> bool {
    let actual = csrf_hash(token);
    actual.as_bytes().ct_eq(expected_hash.as_bytes()).into()
}

pub async fn create(rb: &RBatis, claims: &Claims, csrf_token: &str) -> Result<(), String> {
    rb.exec(
        "DELETE FROM auth_sessions WHERE expires_at <= ?",
        vec![rbs::value!(claims.iat as i64)],
    )
    .await
    .map_err(|error| format!("clean expired auth sessions failed: {error}"))?;
    rb.exec(
        "INSERT INTO auth_sessions (jti, username, role, csrf_hash, issued_at, expires_at, revoked_at) VALUES (?, ?, ?, ?, ?, ?, NULL)",
        vec![
            rbs::value!(claims.jti.clone()),
            rbs::value!(claims.sub.clone()),
            rbs::value!(claims.role.clone()),
            rbs::value!(csrf_hash(csrf_token)),
            rbs::value!(claims.iat as i64),
            rbs::value!(claims.exp as i64),
        ],
    )
    .await
    .map_err(|error| format!("save auth session failed: {error}"))?;
    Ok(())
}

pub async fn find_active(
    rb: &RBatis,
    claims: &Claims,
    now: i64,
) -> Result<Option<AuthSession>, String> {
    let mut sessions: Vec<AuthSession> = rb
        .query_decode(
            "SELECT jti, username, role, csrf_hash, issued_at, expires_at, revoked_at FROM auth_sessions WHERE jti = ? AND revoked_at IS NULL AND expires_at > ?",
            vec![rbs::value!(claims.jti.clone()), rbs::value!(now)],
        )
        .await
        .map_err(|error| format!("query auth session failed: {error}"))?;
    let session = sessions.pop().filter(|session| {
        session.username == claims.sub
            && session.role == claims.role
            && session.expires_at == claims.exp as i64
    });
    Ok(session)
}

pub async fn revoke(rb: &RBatis, jti: &str, now: i64) -> Result<(), String> {
    rb.exec(
        "UPDATE auth_sessions SET revoked_at = ? WHERE jti = ? AND revoked_at IS NULL",
        vec![rbs::value!(now), rbs::value!(jti)],
    )
    .await
    .map_err(|error| format!("revoke auth session failed: {error}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbdc_sqlite::SqliteDriver;

    async fn sqlite() -> RBatis {
        let rb = RBatis::new();
        rb.init(SqliteDriver {}, "sqlite://:memory:")
            .expect("initialize SQLite");
        init_store(&rb).await.expect("create auth session schema");
        rb
    }

    fn claims() -> Claims {
        Claims {
            sub: "alice".to_string(),
            role: "viewer".to_string(),
            iat: 100,
            exp: 200,
            jti: "session-1".to_string(),
            iss: "issuer".to_string(),
            aud: "audience".to_string(),
        }
    }

    #[tokio::test]
    async fn sqlite_session_can_be_revoked_immediately() {
        let rb = sqlite().await;
        let claims = claims();
        create(&rb, &claims, "csrf-secret")
            .await
            .expect("save session");

        let session = find_active(&rb, &claims, 150)
            .await
            .expect("query session")
            .expect("active session");
        assert!(verify_csrf("csrf-secret", &session.csrf_hash));
        assert!(!verify_csrf("wrong", &session.csrf_hash));

        revoke(&rb, &claims.jti, 151).await.expect("revoke session");
        assert!(find_active(&rb, &claims, 152)
            .await
            .expect("query revoked")
            .is_none());
    }

    #[tokio::test]
    async fn sqlite_session_rejects_role_mismatch_and_expiration() {
        let rb = sqlite().await;
        let claims = claims();
        create(&rb, &claims, "csrf-secret")
            .await
            .expect("save session");

        let mut changed_role = claims.clone();
        changed_role.role = "admin".to_string();
        assert!(find_active(&rb, &changed_role, 150)
            .await
            .expect("role query")
            .is_none());
        assert!(find_active(&rb, &claims, 200)
            .await
            .expect("expiry query")
            .is_none());
    }
}
