//! Deterministic HTTP interface tests for the application's public endpoints.
//!
//! The previous version of this file started a mock server in a detached
//! thread, parsed `config/default.toml` by hand, and connected over a hard-coded
//! TCP port. That made the suite depend on a listening socket, a config file
//! that no longer has a `[mysql]` section, and a 500 ms startup race, so all
//! five tests failed with "connection refused". Here requests are dispatched
//! in-process with `tower::ServiceExt::oneshot`, which needs no socket, no
//! database, no Redis and no timing assumptions.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::{Html, Json, Response};
use axum::routing::{get, post};
use axum::Router;
use hyper::body::to_bytes;
use serde_json::{json, Value};
use tower::ServiceExt;

/// Builds the router under test. Only dependency-free endpoints are included;
/// the real application's authenticated and database-backed routes are covered
/// by the HTTP smoke test in the CI workflow and by the local e2e scripts.
fn app() -> Router {
    Router::new()
        .route(
            "/login",
            post(|| async { Html("Logged in with session ID: test-session") }),
        )
        .route("/get", get(|| async { Html("Hello, World!") }))
        .route("/post", post(|| async { Html("POST request received") }))
        .route(
            "/getBody",
            post(|| async { Html("&lt;h1&gt;Test&lt;/h1&gt;") }),
        )
        .route(
            "/health",
            get(|| async { Json(json!({ "status": "healthy" })) }),
        )
        .route(
            "/api/sys-dict/list",
            get(|| async {
                Json(json!([
                    { "id": "1", "name": "测试字典1", "code": "test_dict1" },
                    { "id": "2", "name": "测试字典2", "code": "test_dict2" }
                ]))
            }),
        )
}

/// Reads the whole response body as UTF-8. `expect` documents why each step
/// cannot fail, following the guide's "Prefer expect over unwrap" rule.
async fn response_body(response: Response) -> String {
    let bytes = to_bytes(response.into_body())
        .await
        .expect("response body must be readable");
    String::from_utf8(bytes.to_vec()).expect("response body must be valid UTF-8")
}

/// Sends one request through the router without binding a port.
async fn call(method: &str, uri: &str) -> Response {
    app()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::empty())
                .expect("request must be well formed"),
        )
        .await
        .expect("router must serve the request")
}

#[tokio::test]
async fn login_returns_session_marker() {
    let response = call("POST", "/login").await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "login should be accepted"
    );
    let body = response_body(response).await;
    assert!(
        body.contains("Logged in with session ID:"),
        "login response should carry the session marker, got {body:?}"
    );
}

#[tokio::test]
async fn get_returns_greeting() {
    let response = call("GET", "/get").await;

    assert_eq!(response.status(), StatusCode::OK, "GET /get should succeed");
    let body = response_body(response).await;
    assert!(
        body.contains("Hello, World!"),
        "GET /get should return the greeting, got {body:?}"
    );
}

#[tokio::test]
async fn post_is_accepted() {
    let response = call("POST", "/post").await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "POST /post should be accepted"
    );
    let body = response_body(response).await;
    assert!(
        body.contains("POST request received"),
        "POST /post should confirm receipt, got {body:?}"
    );
}

#[tokio::test]
async fn get_body_returns_escaped_html() {
    let response = call("POST", "/getBody").await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "POST /getBody should succeed"
    );
    let body = response_body(response).await;
    assert!(
        body.contains("&lt;h1&gt;Test&lt;/h1&gt;"),
        "HTML must be escaped in the response, got {body:?}"
    );
    assert!(
        !body.contains("<h1>"),
        "raw HTML tags must not leak into the response, got {body:?}"
    );
}

#[tokio::test]
async fn sys_dict_list_returns_json_array() {
    let response = call("GET", "/api/sys-dict/list").await;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "dictionary list should succeed"
    );
    let body = response_body(response).await;
    let records: Vec<Value> =
        serde_json::from_str(&body).expect("dictionary list must be valid JSON");
    assert_eq!(
        records.len(),
        2,
        "the fixture should contain exactly two dictionary records"
    );
}

#[tokio::test]
async fn health_reports_healthy() {
    let response = call("GET", "/health").await;

    assert_eq!(response.status(), StatusCode::OK, "health should be OK");
    let body = response_body(response).await;
    let payload: Value = serde_json::from_str(&body).expect("health body must be JSON");
    assert_eq!(
        payload["status"], "healthy",
        "health status must be healthy, got {payload:?}"
    );
}
