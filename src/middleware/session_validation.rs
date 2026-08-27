use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    http::{header::COOKIE, HeaderMap, Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use subtle::ConstantTimeEq;

use crate::{auth_session, config::Config, jwt};

pub const ACCESS_COOKIE: &str = "access_token";
pub const CSRF_COOKIE: &str = "csrf_token";
pub const CSRF_HEADER: &str = "x-csrf-token";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CredentialSource {
    Bearer,
    Cookie,
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(COOKIE)
        .and_then(|header| header.to_str().ok())
        .and_then(|cookie| {
            cookie.split(';').find_map(|part| {
                let (key, value) = part.trim().split_once('=')?;
                (key == name).then(|| value.to_owned())
            })
        })
}

fn extract_token(headers: &HeaderMap) -> Option<(String, CredentialSource)> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .map(|token| (token.to_owned(), CredentialSource::Bearer))
        .or_else(|| {
            cookie_value(headers, ACCESS_COOKIE).map(|token| (token, CredentialSource::Cookie))
        })
}

pub fn origin_allowed(headers: &HeaderMap, config: &Config) -> bool {
    let Some(origin) = headers.get(axum::http::header::ORIGIN) else {
        // 非浏览器客户端通常没有 Origin；cookie 请求仍必须持有 CSRF secret。
        return true;
    };
    let Ok(origin) = origin.to_str() else {
        return false;
    };
    config
        .security
        .allowed_origins
        .iter()
        .any(|allowed| allowed == origin)
}

fn is_unsafe_method(method: &Method) -> bool {
    !matches!(*method, Method::GET | Method::HEAD | Method::OPTIONS)
}

fn requires_admin(method: &Method, path: &str) -> bool {
    path.ends_with("/insert")
        || path.ends_with("/update")
        || path.ends_with("/delete")
        || (is_unsafe_method(method) && path.starts_with("/api/image/"))
}

fn now_seconds() -> Result<i64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .map_err(|error| format!("system time error: {error}"))
}

fn error(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(serde_json::json!({
            "code": status.as_u16(),
            "status": "error",
            "message": message
        })),
    )
        .into_response()
}

pub async fn validate_session<B>(mut req: Request<B>, next: Next<B>) -> Response {
    let config = match req.extensions().get::<Config>().cloned() {
        Some(config) => config,
        None => return error(StatusCode::INTERNAL_SERVER_ERROR, "认证配置不可用"),
    };
    let session_store = match req
        .extensions()
        .get::<auth_session::SessionStore>()
        .cloned()
    {
        Some(store) => store,
        None => return error(StatusCode::INTERNAL_SERVER_ERROR, "认证会话存储不可用"),
    };
    let (token, source) = match extract_token(req.headers()) {
        Some(value) => value,
        None => return error(StatusCode::UNAUTHORIZED, "缺少登录凭据"),
    };
    let claims = match jwt::validate(&token, &config.jwt) {
        Ok(claims) => claims,
        Err(reason) => {
            tracing::warn!(%reason, "JWT rejected");
            return error(StatusCode::UNAUTHORIZED, "登录凭据无效或已过期");
        }
    };
    let now = match now_seconds() {
        Ok(now) => now,
        Err(reason) => {
            tracing::error!(%reason, "Clock unavailable during authentication");
            return error(StatusCode::INTERNAL_SERVER_ERROR, "认证服务不可用");
        }
    };
    let session = match auth_session::find_active(&session_store, &claims, now).await {
        Ok(Some(session)) => session,
        Ok(None) => return error(StatusCode::UNAUTHORIZED, "会话已撤销、已过期或权限已变化"),
        Err(reason) => {
            tracing::error!(%reason, "Auth session lookup failed");
            return error(StatusCode::INTERNAL_SERVER_ERROR, "认证服务不可用");
        }
    };

    if claims.role != "admin" && claims.role != "viewer" {
        return error(StatusCode::FORBIDDEN, "会话包含未知角色");
    }
    if requires_admin(req.method(), req.uri().path()) && claims.role != "admin" {
        return error(StatusCode::FORBIDDEN, "当前角色无权执行此操作");
    }

    if source == CredentialSource::Cookie && is_unsafe_method(req.method()) {
        if !origin_allowed(req.headers(), &config) {
            return error(StatusCode::FORBIDDEN, "请求来源不受信任");
        }
        let cookie_token = cookie_value(req.headers(), CSRF_COOKIE);
        let header_token = req
            .headers()
            .get(CSRF_HEADER)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let csrf_valid = match (cookie_token, header_token) {
            (Some(cookie), Some(header)) => {
                bool::from(cookie.as_bytes().ct_eq(header.as_bytes()))
                    && auth_session::verify_csrf(&header, &session.csrf_hash)
            }
            _ => false,
        };
        if !csrf_valid {
            return error(StatusCode::FORBIDDEN, "CSRF 校验失败");
        }
    }

    req.extensions_mut().insert(claims);
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutating_paths_require_admin_but_queries_do_not() {
        assert!(requires_admin(&Method::POST, "/category/insert"));
        assert!(requires_admin(&Method::POST, "/category/update"));
        assert!(requires_admin(&Method::POST, "/category/delete"));
        assert!(requires_admin(&Method::POST, "/api/image/photo.jpg"));
        assert!(!requires_admin(&Method::GET, "/api/image/photo.jpg"));
        assert!(!requires_admin(&Method::POST, "/category/select-page"));
        assert!(!requires_admin(&Method::POST, "/api/logout"));
    }

    #[test]
    fn bearer_takes_precedence_and_cookie_parser_is_exact() {
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            "not_access_token=x; access_token=cookie-token; csrf_token=csrf"
                .parse()
                .unwrap(),
        );
        headers.insert(
            axum::http::header::AUTHORIZATION,
            "Bearer api-token".parse().unwrap(),
        );
        assert_eq!(
            extract_token(&headers),
            Some(("api-token".to_string(), CredentialSource::Bearer))
        );
        assert_eq!(cookie_value(&headers, CSRF_COOKIE).as_deref(), Some("csrf"));
    }
}
