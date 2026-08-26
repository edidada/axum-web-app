use ammonia::clean;
use axum::{response::Html, Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestBody {
    html_code: String,
}
pub async fn get_handler() -> Html<String> {
    tracing::info!("get {}", "get_handler");
    let html_code = "<h1>Hello, World!</h1><script>alert('XSS');</script>";
    let sanitized_html = clean(html_code);
    Html(sanitized_html)
}

pub async fn get_body_handler(Json(payload): Json<RequestBody>) -> Html<String> {
    tracing::info!("get_body_handler {:?}", payload);
    let sanitized_html = clean(&payload.html_code);
    Html(sanitized_html)
}
