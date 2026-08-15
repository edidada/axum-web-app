use axum::{response::{Html, IntoResponse, Response}, http::StatusCode, Json, extract::Extension};
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
    Extension(config): Extension<Config>,
) -> Response {

    // Validate username and password
    if payload.username == config.auth.username && payload.password == config.auth.password {
        let mut conn = match redis_client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to connect to Redis: {error}")).into_response(),
        };
        let session_id = Uuid::new_v4().to_string();
        let expiration = config.redis.expiration;
        let key = format!("{}{}", config.redis.key_prefix, session_id);

        if let Err(error) = conn.set_ex::<_, _, ()>(&key, &payload.username, expiration as u64).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save session: {error}")).into_response();
        }

        Html(format!("Logged in with session ID: {}", session_id)).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, Html("Invalid username or password".to_string())).into_response()
    }
}
