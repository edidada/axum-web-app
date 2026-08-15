use axum::{
    extract::Extension,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use redis::AsyncCommands;
use serde::Deserialize;

use crate::config::Config;
use crate::jwt;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
    Extension(redis_client): Extension<redis::Client>,
    Extension(config): Extension<Config>,
) -> Response {
    if payload.username != config.auth.username || payload.password != config.auth.password {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"code": 1, "status": "error", "message": "用户名或密码错误"})),
        ).into_response();
    }

    let mut conn = match redis_client.get_multiplexed_async_connection().await {
        Ok(conn) => conn,
        Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Redis连接失败: {error}")).into_response(),
    };
    let (access_token, claims) = match jwt::issue(&payload.username, &config.jwt) {
        Ok(token) => token,
        Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, error).into_response(),
    };
    let key = format!("{}{}", config.redis.key_prefix, access_token);
    if let Err(error) = conn.set_ex::<_, _, ()>(&key, &payload.username, config.redis.expiration as u64).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("保存会话失败: {error}")).into_response();
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(&format!("session={access_token}; Path=/; HttpOnly; Max-Age={}", config.redis.expiration)).expect("valid session cookie"),
    );
    (StatusCode::OK, headers, Json(serde_json::json!({
        "code": 0,
        "status": "success",
        "session_id": access_token,
        "access_token": access_token,
        "token_type": "Bearer",
        "expires_in": config.jwt.expiration,
        "jti": claims.jti
    }))).into_response()
}
