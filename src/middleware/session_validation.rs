use axum::{
    http::{Request, StatusCode},
    response::IntoResponse,
    middleware::Next,
};
use redis::{AsyncCommands, Client};
use crate::config::Config;

pub async fn validate_session<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let redis_client = match req.extensions().get::<Client>().cloned() {
        Some(client) => client,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let config = match req.extensions().get::<Config>().cloned() {
        Some(config) => config,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // Extract session_id before borrowing req mutably
    let session_id = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    if let Some(session_id) = session_id {
        let mut conn = match redis_client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        let key = format!("{}{}", config.redis.key_prefix, session_id);
        let username: Option<String> = match conn.get(&key).await {
            Ok(username) => username,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if username.is_some() {
            req.extensions_mut().insert(session_id);
            return next.run(req).await;
        }
    }

    (StatusCode::UNAUTHORIZED, "Invalid session").into_response()
}
