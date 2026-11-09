use ammonia::clean;
use axum::extract::Form;
use axum::response::Html;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    custom_username: String,
    custom_password: String,
}

pub async fn post_handler(Form(input): Form<Input>) -> Html<String> {
    let html_code = "<h1>Hello, World!</h1><script>alert('XSS');</script>";
    let encoded_html = clean(html_code);
    let username = clean(&input.custom_username);
    let password_length = input.custom_password.chars().count();
    tracing::info!(username = %username, password_length, "Processed form submission");

    Html(format!(
        "<html><body><h2>Formatted HTML Code:</h2><pre>{}</pre><p>Submission received for: {}</p></body></html>",
        encoded_html, username
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn post_handler_reads_form_fields_without_exposing_password() {
        let response = post_handler(Form(Input {
            custom_username: "<b>Alice</b>".to_string(),
            custom_password: "secret-password".to_string(),
        }))
        .await;

        assert!(
            response.0.contains("Submission received for: <b>Alice</b>"),
            "the sanitized username should appear in the response"
        );
        assert!(
            !response.0.contains("secret-password"),
            "the password must never be echoed back"
        );
        assert!(
            !response.0.contains("<script>"),
            "script tags must be stripped by the sanitizer"
        );
    }
}
