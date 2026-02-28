use axum::{response::Html, Json, extract::Extension};
use serde::Deserialize;
use redis::AsyncCommands;
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
) -> Html<String> {
    // Load configuration
    let config = Config::from_file();

    // Validate username and password
    if payload.username == config.auth.username && payload.password == config.auth.password {
        let mut conn = redis_client.get_async_connection().await.unwrap();
        let session_id = Uuid::new_v4().to_string();
        let expiration = config.redis.expiration;
        let key = format!("{}{}", config.redis.key_prefix, session_id);

        let _: () = conn.set_ex(&key, &payload.username, expiration).await.unwrap();

        Html(format!("Logged in with session ID: {}", session_id))
    } else {
        Html("Invalid username or password".to_string())
    }
}