use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{Request, StatusCode},
    response::IntoResponse,
    middleware::Next,
    Extension,
};
use redis::{AsyncCommands, Client};
use std::task::{Context, Poll};
use tower::Service;

pub struct ValidSession(pub String);

#[async_trait]
impl<B> FromRequest<B> for ValidSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let redis_client = req.extensions().get::<Client>().cloned().ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Redis client missing"))?;

        let session_id = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|auth| auth.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header"))?;

        let mut conn = redis_client.get_async_connection().await.map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to Redis")
        })?;

        let username: Option<String> = conn.get(session_id).await.map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to query Redis")
        })?;

        if let Some(username) = username {
            let session_id = session_id.to_string();
            req.extensions_mut().insert(session_id.clone());
            Ok(ValidSession(username))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Invalid session"))
        }
    }
}

pub async fn validate_session<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let redis_client = req.extensions().get::<Client>().cloned().unwrap();

    // Extract session_id before borrowing req mutably
    let session_id = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    if let Some(session_id) = session_id {
        let mut conn = redis_client.get_async_connection().await.unwrap();
        let username: Option<String> = conn.get(&session_id).await.unwrap();

        if username.is_some() {
            req.extensions_mut().insert(session_id);
            return next.run(req).await;
        }
    }

    (StatusCode::UNAUTHORIZED, "Invalid session").into_response()
}