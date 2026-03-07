use axum::{Json, extract::Extension, response::IntoResponse, http::HeaderMap, http::HeaderValue};
use serde::Deserialize;
use redis::AsyncCommands;
use uuid::Uuid;
use crate::config::Config;
use serde_json;
use log;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
    Extension(redis_client): Extension<redis::Client>,
) -> impl IntoResponse {
    // Load configuration
    let config = Config::from_file();

    // Log the entire configuration
    log::info!("Configuration: {}", serde_json::to_string_pretty(&config).unwrap());

    // Validate username and password
    if payload.username == config.auth.username && payload.password == config.auth.password {
        let mut conn = redis_client.get_async_connection().await.unwrap();
        let session_id = Uuid::new_v4().to_string();
        let expiration = config.redis.expiration;
        let key = format!("{}{}", config.redis.key_prefix, session_id);

        let _: () = conn.set_ex(&key, &payload.username, expiration).await.unwrap();

        // Create headers with Set-Cookie
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::SET_COOKIE,
            HeaderValue::from_str(&format!("session={}; Path=/; HttpOnly; Max-Age={}", session_id, expiration)).unwrap(),
        );

        (
            axum::http::StatusCode::OK,
            headers,
            Json(serde_json::json!({
                "code": 0,
                "status": "success",
                "session_id": session_id
            }))
        )
    } else {
        // Create empty headers
        let headers = HeaderMap::new();

        (
            axum::http::StatusCode::UNAUTHORIZED,
            headers,
            Json(serde_json::json!({
                "code": 1,
                "status": "error",
                "message": "Invalid username or password"
            }))
        )
    }
}