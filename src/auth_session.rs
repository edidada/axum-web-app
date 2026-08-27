use rbatis::RBatis;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::{
    config::{SessionBackend, SessionConfig},
    jwt::Claims,
};

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

#[derive(Clone)]
pub enum SessionStore {
    Database(&'static RBatis),
    Redis {
        client: redis::Client,
        key_prefix: String,
    },
}

impl SessionStore {
    pub async fn initialize(config: &SessionConfig, rb: &'static RBatis) -> Result<Self, String> {
        match config.backend {
            SessionBackend::Database => {
                init_database_store(rb).await?;
                Ok(Self::Database(rb))
            }
            SessionBackend::Redis => {
                let client = redis::Client::open(config.redis_url.as_str())
                    .map_err(|error| format!("invalid Redis session URL: {error}"))?;
                let mut connection = client
                    .get_async_connection()
                    .await
                    .map_err(|error| format!("connect to Redis session store failed: {error}"))?;
                let pong: String = redis::cmd("PING")
                    .query_async(&mut connection)
                    .await
                    .map_err(|error| format!("Redis session store ping failed: {error}"))?;
                if pong != "PONG" {
                    return Err(format!(
                        "Redis session store returned unexpected PING: {pong}"
                    ));
                }
                Ok(Self::Redis {
                    client,
                    key_prefix: config.redis_key_prefix.clone(),
                })
            }
        }
    }

    pub fn backend_name(&self) -> &'static str {
        match self {
            Self::Database(_) => "database",
            Self::Redis { .. } => "redis",
        }
    }

    fn redis_key(key_prefix: &str, jti: &str) -> String {
        format!("{key_prefix}{jti}")
    }
}

async fn init_database_store(rb: &RBatis) -> Result<(), String> {
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

pub async fn create(store: &SessionStore, claims: &Claims, csrf_token: &str) -> Result<(), String> {
    let session = AuthSession {
        jti: claims.jti.clone(),
        username: claims.sub.clone(),
        role: claims.role.clone(),
        csrf_hash: csrf_hash(csrf_token),
        issued_at: claims.iat as i64,
        expires_at: claims.exp as i64,
        revoked_at: None,
    };

    match store {
        SessionStore::Database(rb) => {
            rb.exec(
                "DELETE FROM auth_sessions WHERE expires_at <= ?",
                vec![rbs::value!(claims.iat as i64)],
            )
            .await
            .map_err(|error| format!("clean expired auth sessions failed: {error}"))?;
            rb.exec(
                "INSERT INTO auth_sessions (jti, username, role, csrf_hash, issued_at, expires_at, revoked_at) VALUES (?, ?, ?, ?, ?, ?, NULL)",
                vec![
                    rbs::value!(session.jti),
                    rbs::value!(session.username),
                    rbs::value!(session.role),
                    rbs::value!(session.csrf_hash),
                    rbs::value!(session.issued_at),
                    rbs::value!(session.expires_at),
                ],
            )
            .await
            .map_err(|error| format!("save auth session failed: {error}"))?;
        }
        SessionStore::Redis { client, key_prefix } => {
            let ttl = claims
                .exp
                .checked_sub(claims.iat)
                .filter(|ttl| *ttl > 0)
                .ok_or_else(|| "auth session must expire after it is issued".to_string())?;
            let payload = serde_json::to_string(&session)
                .map_err(|error| format!("serialize Redis auth session failed: {error}"))?;
            let mut connection = client
                .get_async_connection()
                .await
                .map_err(|error| format!("connect to Redis session store failed: {error}"))?;
            redis::cmd("SET")
                .arg(SessionStore::redis_key(key_prefix, &claims.jti))
                .arg(payload)
                .arg("EX")
                .arg(ttl)
                .query_async::<_, ()>(&mut connection)
                .await
                .map_err(|error| format!("save Redis auth session failed: {error}"))?;
        }
    }
    Ok(())
}

pub async fn find_active(
    store: &SessionStore,
    claims: &Claims,
    now: i64,
) -> Result<Option<AuthSession>, String> {
    let session = match store {
        SessionStore::Database(rb) => {
            let mut sessions: Vec<AuthSession> = rb
                .query_decode(
                    "SELECT jti, username, role, csrf_hash, issued_at, expires_at, revoked_at FROM auth_sessions WHERE jti = ? AND revoked_at IS NULL AND expires_at > ?",
                    vec![rbs::value!(claims.jti.clone()), rbs::value!(now)],
                )
                .await
                .map_err(|error| format!("query auth session failed: {error}"))?;
            sessions.pop()
        }
        SessionStore::Redis { client, key_prefix } => {
            let mut connection = client
                .get_async_connection()
                .await
                .map_err(|error| format!("connect to Redis session store failed: {error}"))?;
            let payload: Option<String> = redis::cmd("GET")
                .arg(SessionStore::redis_key(key_prefix, &claims.jti))
                .query_async(&mut connection)
                .await
                .map_err(|error| format!("query Redis auth session failed: {error}"))?;
            payload
                .map(|payload| {
                    serde_json::from_str(&payload)
                        .map_err(|error| format!("decode Redis auth session failed: {error}"))
                })
                .transpose()?
        }
    };

    Ok(session.filter(|session| {
        session.username == claims.sub
            && session.role == claims.role
            && session.expires_at == claims.exp as i64
            && session.expires_at > now
            && session.revoked_at.is_none()
    }))
}

pub async fn revoke(store: &SessionStore, jti: &str, now: i64) -> Result<(), String> {
    match store {
        SessionStore::Database(rb) => {
            rb.exec(
                "UPDATE auth_sessions SET revoked_at = ? WHERE jti = ? AND revoked_at IS NULL",
                vec![rbs::value!(now), rbs::value!(jti)],
            )
            .await
            .map_err(|error| format!("revoke auth session failed: {error}"))?;
        }
        SessionStore::Redis { client, key_prefix } => {
            let mut connection = client
                .get_async_connection()
                .await
                .map_err(|error| format!("connect to Redis session store failed: {error}"))?;
            redis::cmd("DEL")
                .arg(SessionStore::redis_key(key_prefix, jti))
                .query_async::<_, usize>(&mut connection)
                .await
                .map_err(|error| format!("revoke Redis auth session failed: {error}"))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbdc_sqlite::SqliteDriver;

    async fn sqlite() -> SessionStore {
        let rb = RBatis::new();
        rb.init(SqliteDriver {}, "sqlite://:memory:")
            .expect("initialize SQLite");
        let rb = Box::leak(Box::new(rb));
        init_database_store(rb)
            .await
            .expect("create auth session schema");
        SessionStore::Database(rb)
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
    async fn database_session_can_be_revoked_immediately() {
        let store = sqlite().await;
        let claims = claims();
        create(&store, &claims, "csrf-secret")
            .await
            .expect("save session");

        let session = find_active(&store, &claims, 150)
            .await
            .expect("query session")
            .expect("active session");
        assert!(verify_csrf("csrf-secret", &session.csrf_hash));
        assert!(!verify_csrf("wrong", &session.csrf_hash));

        revoke(&store, &claims.jti, 151)
            .await
            .expect("revoke session");
        assert!(find_active(&store, &claims, 152)
            .await
            .expect("query revoked")
            .is_none());
    }

    #[tokio::test]
    async fn database_session_rejects_role_mismatch_and_expiration() {
        let store = sqlite().await;
        let claims = claims();
        create(&store, &claims, "csrf-secret")
            .await
            .expect("save session");

        let mut changed_role = claims.clone();
        changed_role.role = "admin".to_string();
        assert!(find_active(&store, &changed_role, 150)
            .await
            .expect("role query")
            .is_none());
        assert!(find_active(&store, &claims, 200)
            .await
            .expect("expiry query")
            .is_none());
    }
}
