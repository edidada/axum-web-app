use axum::response::Html;

pub async fn get_handler() -> Html<&'static str> {
    let html_code = "<h1>Hello, World!</h1><script>alert('XSS');</script>";
    let encoded_html = axum::extract::Extension::new(html_code);
    Html(encoded_html)
}