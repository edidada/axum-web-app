use axum::{
    extract::Extension,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use redis::AsyncCommands;
use serde::Deserialize;
use uuid::Uuid;

use crate::config::Config;

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
    let session_id = Uuid::new_v4().to_string();
    let key = format!("{}{}", config.redis.key_prefix, session_id);
    if let Err(error) = conn.set_ex::<_, _, ()>(&key, &payload.username, config.redis.expiration as u64).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("保存会话失败: {error}")).into_response();
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(&format!("session={session_id}; Path=/; HttpOnly; Max-Age={}", config.redis.expiration)).expect("valid session cookie"),
    );
    (StatusCode::OK, headers, Json(serde_json::json!({
        "code": 0,
        "status": "success",
        "session_id": session_id
    }))).into_response()
}
