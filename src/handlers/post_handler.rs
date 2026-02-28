use ammonia::clean;
use axum::response::Html;
use axum::extract::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    custom_username: String,
    custom_password: String,
}

pub async fn post_handler(Form(_input): Form<Input>) -> Html<String> {
    let html_code = "<h1>Hello, World!</h1><script>alert('XSS');</script>";
    let encoded_html = clean(html_code);
    Html(format!(
        "<html><body><h2>Formatted HTML Code:</h2><pre>{}</pre></body></html>",
        encoded_html
    ))
}