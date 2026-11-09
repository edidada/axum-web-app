use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;
use tracing::{debug, info};

pub async fn log_request<B>(req: Request<B>, next: Next<B>) -> Response
where
    B: Send,
{
    let start = Instant::now();
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    // 记录请求头
    debug!("Request: {} {}", method, path);
    debug!("Headers: {:?}", req.headers());

    // 处理请求
    let response = next.run(req).await;

    // 记录响应
    let status = response.status();
    let duration = start.elapsed();
    info!(
        "Response: {} {} - {} in {:?}",
        method, path, status, duration
    );

    // 记录响应头
    debug!("Response Headers: {:?}", response.headers());

    response
}
