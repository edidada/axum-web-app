use axum::response::Html;
use ammonia::clean;

pub async fn get_handler() -> Html<String> {
    let html_code = "<h1>Hello, World!</h1><script>alert('XSS');</script>";
    let sanitized_html = clean(html_code);
    Html(sanitized_html)
}