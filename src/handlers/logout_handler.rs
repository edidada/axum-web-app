use crate::config::Config;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::Html;
use redis::AsyncCommands;

pub async fn logout_handler(
    Extension(redis_client): Extension<redis::Client>,
    Extension(config): Extension<Config>,
    Extension(session_id): Extension<String>,
) -> Result<Html<String>, (StatusCode, String)> {
    let mut conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to connect to Redis".to_string(),
            )
        })?;

    let key = format!("{}{}", config.redis.key_prefix, session_id);
    let _: () = conn.del(&key).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete session".to_string(),
        )
    })?;

    Ok(Html("Logged out successfully".to_string()))
}
