use axum::{
    http::{header::COOKIE, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use redis::{AsyncCommands, Client};

use crate::config::Config;

fn extract_session_id<B>(req: &Request<B>) -> Option<String> {
    req.headers().get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(str::to_owned)
        .or_else(|| {
            req.headers().get(COOKIE)
                .and_then(|header| header.to_str().ok())
                .and_then(|cookie| cookie.split(';').find_map(|part| {
                    part.trim().strip_prefix("session=").map(str::to_owned)
                }))
        })
}

pub async fn validate_session<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let redis_client = match req.extensions().get::<Client>().cloned() {
        Some(client) => client,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let config = match req.extensions().get::<Config>().cloned() {
        Some(config) => config,
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let session_id = match extract_session_id(&req) {
        Some(session_id) => session_id,
        None => return (StatusCode::UNAUTHORIZED, "缺少或无效的会话").into_response(),
    };

    let mut conn = match redis_client.get_multiplexed_async_connection().await {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Redis连接失败").into_response(),
    };
    let key = format!("{}{}", config.redis.key_prefix, session_id);
    let username: Option<String> = match conn.get(&key).await {
        Ok(username) => username,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Redis查询失败").into_response(),
    };
    if username.is_none() {
        return (StatusCode::UNAUTHORIZED, "会话无效，请重新登录").into_response();
    }

    req.extensions_mut().insert(session_id);
    next.run(req).await
}
