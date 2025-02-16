use axum::{response::Html, extract::Extension, http::StatusCode};
use redis::AsyncCommands;

pub async fn logout_handler(
    Extension(redis_client): Extension<redis::Client>,
    Extension(session_id): Extension<String>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut conn = redis_client.get_async_connection().await.map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to Redis".to_string())
    })?;

    let _: () = conn.del(session_id).await.map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete session".to_string())
    })?;

    Ok(Html("Logged out successfully".to_string()))
}