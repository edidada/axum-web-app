use axum::{
    extract::Extension,
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use uuid::Uuid;

use crate::{
    auth_session,
    config::Config,
    jwt::{self, Claims},
    middleware::session_validation::{origin_allowed, ACCESS_COOKIE, CSRF_COOKIE},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

fn password_matches(supplied: &str, configured: &str) -> bool {
    let supplied = Sha256::digest(supplied.as_bytes());
    let configured = Sha256::digest(configured.as_bytes());
    bool::from(supplied.as_slice().ct_eq(configured.as_slice()))
}

fn cookie(name: &str, value: &str, max_age: usize, http_only: bool, secure: bool) -> String {
    let mut cookie = format!("{name}={value}; Path=/; SameSite=Lax; Max-Age={max_age}");
    if http_only {
        cookie.push_str("; HttpOnly");
    }
    if secure {
        cookie.push_str("; Secure");
    }
    cookie
}

fn json_error(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(serde_json::json!({"code": status.as_u16(), "status": "error", "message": message})),
    )
        .into_response()
}

pub async fn login_handler(
    headers: HeaderMap,
    Extension(session_store): Extension<auth_session::SessionStore>,
    Extension(config): Extension<Config>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    if !origin_allowed(&headers, &config) {
        return json_error(StatusCode::FORBIDDEN, "请求来源不受信任");
    }
    if payload.username != config.auth.username
        || !password_matches(&payload.password, &config.auth.password)
    {
        return json_error(StatusCode::UNAUTHORIZED, "用户名或密码错误");
    }

    let (access_token, claims) = match jwt::issue(&payload.username, &config.auth.role, &config.jwt)
    {
        Ok(token) => token,
        Err(reason) => {
            tracing::error!(%reason, "JWT issue failed");
            return json_error(StatusCode::INTERNAL_SERVER_ERROR, "登录服务不可用");
        }
    };
    let csrf_token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    if let Err(reason) = auth_session::create(&session_store, &claims, &csrf_token).await {
        tracing::error!(%reason, "Auth session creation failed");
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "登录服务不可用");
    }

    let access_cookie = cookie(
        ACCESS_COOKIE,
        &access_token,
        config.jwt.expiration,
        true,
        config.security.cookie_secure,
    );
    let csrf_cookie = cookie(
        CSRF_COOKIE,
        &csrf_token,
        config.jwt.expiration,
        false,
        config.security.cookie_secure,
    );
    let (Ok(access_cookie), Ok(csrf_cookie)) = (
        HeaderValue::from_str(&access_cookie),
        HeaderValue::from_str(&csrf_cookie),
    ) else {
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "无法创建安全 cookie");
    };

    let mut response = (
        StatusCode::OK,
        Json(serde_json::json!({
            "code": 0,
            "status": "success",
            "message": "登录成功",
            "username": claims.sub,
            "role": claims.role,
            "expires_in": config.jwt.expiration
        })),
    )
        .into_response();
    response.headers_mut().append(SET_COOKIE, access_cookie);
    response.headers_mut().append(SET_COOKIE, csrf_cookie);
    response
}

pub async fn me_handler(Extension(claims): Extension<Claims>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 0,
        "status": "success",
        "username": claims.sub,
        "role": claims.role,
        "expires_at": claims.exp
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookies_have_browser_security_attributes() {
        let access = cookie(ACCESS_COOKIE, "jwt", 900, true, true);
        assert!(access.contains("HttpOnly"));
        assert!(access.contains("SameSite=Lax"));
        assert!(access.contains("Secure"));

        let csrf = cookie(CSRF_COOKIE, "csrf", 900, false, false);
        assert!(!csrf.contains("HttpOnly"));
    }

    #[test]
    fn password_comparison_uses_digest_and_rejects_wrong_value() {
        assert!(password_matches("secret", "secret"));
        assert!(!password_matches("secret", "different"));
    }
}
