use axum::{
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};

pub async fn add_security_headers<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    headers.insert(
        "referrer-policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        "permissions-policy",
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    headers.insert(
        "content-security-policy",
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data: http: https:; connect-src 'self' http://localhost:9002 http://127.0.0.1:9002; object-src 'none'; base-uri 'self'; frame-ancestors 'none'; form-action 'self'",
        ),
    );
    response
}
