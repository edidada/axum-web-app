use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{Request, StatusCode, header::COOKIE},
    response::IntoResponse,
    middleware::Next,
};
use redis::{AsyncCommands, Client};
use crate::config::Config;

pub struct ValidSession(pub String);

/// 从请求中提取 session_id
/// 优先从 Authorization 头部获取 Bearer token，如果没有则从 Cookie 中获取
fn extract_session_id<B>(req: &RequestParts<B>) -> Option<String> {
    // 首先尝试从 Authorization 头部获取 Bearer token
    let auth_session = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    if auth_session.is_some() {
        return auth_session;
    }

    // 如果没有 Authorization 头部，则从 Cookie 中获取
    req
        .headers()
        .get(COOKIE)
        .and_then(|header| header.to_str().ok())
        .and_then(|cookie| {
            cookie.split(';').find_map(|c| {
                let c = c.trim();
                if c.starts_with("session=") {
                    c.strip_prefix("session=").map(|s| s.to_string())
                } else {
                    None
                }
            })
        })
}

#[async_trait]
impl<B> FromRequest<B> for ValidSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let redis_client = req.extensions().get::<Client>().cloned().ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Redis client missing"))?;
        let config = Config::from_file();

        // 从 Authorization 头部或 Cookie 中获取 session_id
        let session_id = extract_session_id(req)
            .ok_or((StatusCode::UNAUTHORIZED, "缺少或无效的会话"))?;

        let mut conn = redis_client.get_async_connection().await.map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, "Redis连接失败")
        })?;

        let key = format!("{}{}", config.redis.key_prefix, session_id);
        let username: Option<String> = conn.get(&key).await.map_err(|_| {
            (StatusCode::INTERNAL_SERVER_ERROR, "Redis查询失败")
        })?;

        if let Some(username) = username {
            let session_id = session_id.to_string();
            req.extensions_mut().insert(session_id.clone());
            Ok(ValidSession(username))
        } else {
            Err((StatusCode::UNAUTHORIZED, "会话无效"))
        }
    }
}

pub async fn validate_session<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let redis_client = req.extensions().get::<Client>().cloned().unwrap();
    let config = Config::from_file();

    // 从 Authorization 头部或 Cookie 中获取 session_id
    let session_id = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .or_else(|| {
            req
                .headers()
                .get(COOKIE)
                .and_then(|header| header.to_str().ok())
                .and_then(|cookie| {
                    cookie.split(';').find_map(|c| {
                        let c = c.trim();
                        if c.starts_with("session=") {
                            c.strip_prefix("session=").map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                })
        });

    if let Some(session_id) = session_id {
        let mut conn = redis_client.get_async_connection().await.unwrap();
        let key = format!("{}{}", config.redis.key_prefix, session_id);
        let username: Option<String> = conn.get(&key).await.unwrap();

        if username.is_some() {
            req.extensions_mut().insert(session_id);
            return next.run(req).await;
        }
    }

    (StatusCode::UNAUTHORIZED, "会话无效，请重新登录").into_response()
}
