use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::Extension,
    http::{header::SET_COOKIE, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    auth_session,
    config::Config,
    jwt::Claims,
    middleware::session_validation::{ACCESS_COOKIE, CSRF_COOKIE},
};

pub async fn logout_handler(
    Extension(session_store): Extension<auth_session::SessionStore>,
    Extension(config): Extension<Config>,
    Extension(claims): Extension<Claims>,
) -> Response {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    if let Err(reason) = auth_session::revoke(&session_store, &claims.jti, now).await {
        tracing::error!(%reason, "Logout revocation failed");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let secure = if config.security.cookie_secure {
        "; Secure"
    } else {
        ""
    };
    let mut response = Json(serde_json::json!({
        "code": 0,
        "status": "success",
        "message": "退出成功"
    }))
    .into_response();
    for value in [
        format!("{ACCESS_COOKIE}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0{secure}"),
        format!("{CSRF_COOKIE}=; Path=/; SameSite=Lax; Max-Age=0{secure}"),
    ] {
        if let Ok(value) = HeaderValue::from_str(&value) {
            response.headers_mut().append(SET_COOKIE, value);
        }
    }
    response
}
