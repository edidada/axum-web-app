use reqwest::Client;
use tokio;

#[tokio::test]
async fn test_get() {
    let client = Client::new();
    let res = client.get("http://127.0.0.1:9001/get")
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let body = res.text().await.expect("Failed to read response body");
    assert!(body.contains("Hello, World!"));
}

#[tokio::test]
async fn test_post() {
    let client = Client::new();
    let res = client.post("http://127.0.0.1:9001/post")
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
}

#[tokio::test]
async fn test_get_body() {
    let client = Client::new();
    let res = client.post("http://127.0.0.1:9001/getBody")
        .json(&serde_json::json!({ "html_code": "<h1>Test</h1>" }))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let body = res.text().await.expect("Failed to read response body");
    assert!(body.contains("&lt;h1&gt;Test&lt;/h1&gt;"));
}

#[tokio::test]
async fn test_login() {
    let client = Client::new();
    let res = client.post("http://127.0.0.1:9001/login")
        .json(&serde_json::json!({ "username": "user", "password": "password" }))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let body = res.text().await.expect("Failed to read response body");
    assert!(body.contains("Logged in with session ID:"));
}