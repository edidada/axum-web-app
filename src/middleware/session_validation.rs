use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use redis::AsyncCommands;

pub struct ValidSession(pub String);

#[async_trait]
impl<B> FromRequest<B> for ValidSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(redis_client) = Extension::<redis::Client>::from_request(req)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Redis client missing"))?;

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
            Ok(ValidSession(username))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Invalid session"))
        }
    }
}