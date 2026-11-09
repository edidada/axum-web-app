//! HS256 JSON Web Token issuance and validation.

use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::JwtConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject：当前登录用户。
    pub sub: String,
    /// Issued at：签发时间（Unix 秒）。
    pub iat: usize,
    /// Expiration：过期时间（Unix 秒）。
    pub exp: usize,
    /// JWT ID：本次登录的唯一编号，可用于审计或撤销。
    pub jti: String,
}

fn now_seconds() -> Result<usize, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as usize)
        .map_err(|error| format!("system time error: {error}"))
}

/// Returns the first 16 bytes of a token for safe inclusion in logs.
///
/// The full token and the signing secret are never logged.
#[must_use]
fn token_preview(token: &str) -> &str {
    let length = token.len().min(16);
    &token[..length]
}

/// 签发 HS256 JWT。日志只展示算法、claims 和令牌前缀；不会记录密钥或完整 Bearer 令牌。
pub fn issue(username: &str, config: &JwtConfig) -> Result<(String, Claims), String> {
    let iat = now_seconds()?;
    let claims = Claims {
        sub: username.to_owned(),
        iat,
        exp: iat + config.expiration,
        jti: Uuid::new_v4().to_string(),
    };
    let header = Header::new(Algorithm::HS256);
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|error| format!("JWT signing failed: {error}"))?;

    tracing::info!(
        jwt_algorithm = "HS256",
        jwt_header = ?header,
        jwt_claims = ?claims,
        token_prefix = token_preview(&token),
        "JWT issued: header.payload is signed with the configured secret"
    );
    Ok((token, claims))
}

/// 校验 JWT 的算法、签名和 `exp`。校验成功后返回可供业务层使用的 claims。
pub fn validate(token: &str, config: &JwtConfig) -> Result<Claims, String> {
    let header =
        decode_header(token).map_err(|error| format!("JWT header parse failed: {error}"))?;
    if header.alg != Algorithm::HS256 {
        return Err(format!("unsupported JWT algorithm: {:?}", header.alg));
    }

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )
    .map_err(|error| format!("JWT validation failed: {error}"))?;

    tracing::info!(
        jwt_algorithm = ?header.alg,
        jwt_claims = ?data.claims,
        token_prefix = token_preview(token),
        "JWT validated: signature and expiration are valid"
    );
    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JwtConfig {
        JwtConfig {
            secret: "test-jwt-secret".to_string(),
            expiration: 60,
        }
    }

    #[test]
    fn issue_and_validate_hs256_token() {
        let config = test_config();
        let (token, issued_claims) = issue("tester", &config).expect("issue token");
        assert_eq!(
            token.split('.').count(),
            3,
            "JWT must have header.payload.signature"
        );

        let validated_claims = validate(&token, &config).expect("validate token");
        assert_eq!(validated_claims.sub, "tester");
        assert_eq!(validated_claims.jti, issued_claims.jti);
    }

    #[test]
    fn reject_tampered_token() {
        let config = test_config();
        let (mut token, _) = issue("tester", &config).expect("issue token");
        token.push('x');
        assert!(validate(&token, &config).is_err());
    }
}
