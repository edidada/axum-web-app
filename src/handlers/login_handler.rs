use axum::{response::Html, Json, extract::Extension};
use serde::Deserialize;
use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
    Extension(redis_client): Extension<redis::Client>,
) -> Html<String> {
    // Validate username and password (this is just a placeholder)
    if payload.username == "user" && payload.password == "password" {
        let mut conn = redis_client.get_async_connection().await.unwrap();
        let session_id = uuid::Uuid::new_v4().to_string();
        let expiration = 3600; // 1 hour

        let _: () = conn.set_ex(&session_id, &payload.username, expiration).await.unwrap();

        Html(format!("Logged in with session ID: {}", session_id))
    } else {
        Html("Invalid username or password".to_string())
    }
}