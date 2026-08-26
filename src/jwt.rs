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
    /// 当前授权角色；最终权限仍由后端与会话表共同校验。
    pub role: String,
    /// Issuer / Audience 防止一个系统的令牌被另一个系统误用。
    pub iss: String,
    pub aud: String,
}

fn now_seconds() -> Result<usize, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as usize)
        .map_err(|error| format!("system time error: {error}"))
}

/// 签发 HS256 JWT。日志只记录非秘密的会话元数据，不记录密钥或 Bearer 令牌。
pub fn issue(username: &str, role: &str, config: &JwtConfig) -> Result<(String, Claims), String> {
    let iat = now_seconds()?;
    let claims = Claims {
        sub: username.to_owned(),
        iat,
        exp: iat + config.expiration,
        jti: Uuid::new_v4().to_string(),
        role: role.to_owned(),
        iss: config.issuer.clone(),
        aud: config.audience.clone(),
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
        jwt_jti = %claims.jti,
        jwt_subject = %claims.sub,
        jwt_role = %claims.role,
        jwt_expires_at = claims.exp,
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
    validation.set_audience(&[config.audience.as_str()]);
    validation.set_issuer(&[config.issuer.as_str()]);
    validation.set_required_spec_claims(&["exp", "sub", "iat", "jti", "iss", "aud"]);
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )
    .map_err(|error| format!("JWT validation failed: {error}"))?;

    tracing::info!(
        jwt_algorithm = ?header.alg,
        jwt_jti = %data.claims.jti,
        jwt_subject = %data.claims.sub,
        jwt_role = %data.claims.role,
        "JWT validated: signature and expiration are valid"
    );
    if data.claims.sub.is_empty() || data.claims.jti.is_empty() || data.claims.role.is_empty() {
        return Err("JWT contains an empty subject, role, or jti".to_string());
    }
    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JwtConfig {
        JwtConfig {
            secret: "test-jwt-secret".to_string(),
            expiration: 60,
            issuer: "test-issuer".to_string(),
            audience: "test-audience".to_string(),
        }
    }

    #[test]
    fn issue_and_validate_hs256_token() {
        let config = test_config();
        let (token, issued_claims) = issue("tester", "viewer", &config).expect("issue token");
        assert_eq!(
            token.split('.').count(),
            3,
            "JWT must have header.payload.signature"
        );

        let validated_claims = validate(&token, &config).expect("validate token");
        assert_eq!(validated_claims.sub, "tester");
        assert_eq!(validated_claims.jti, issued_claims.jti);
        assert_eq!(validated_claims.role, "viewer");
    }

    #[test]
    fn reject_tampered_token() {
        let config = test_config();
        let (mut token, _) = issue("tester", "viewer", &config).expect("issue token");
        token.push('x');
        assert!(validate(&token, &config).is_err());
    }

    #[test]
    fn reject_token_for_another_audience() {
        let config = test_config();
        let (token, _) = issue("tester", "viewer", &config).expect("issue token");
        let mut other = config.clone();
        other.audience = "another-application".to_string();
        assert!(validate(&token, &other).is_err());
    }
}
