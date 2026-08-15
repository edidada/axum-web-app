use axum::{Extension, Router};
use rbdc_sqlite::driver::SqliteDriver;
use rbatis::rbatis::RBatis;
use redis::Client as RedisClient;
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use std::{error::Error, time::{Duration, Instant}};
use tower::ServiceBuilder;

#[path = "../config.rs"]
mod config;
#[path = "../domain/mod.rs"]
mod domain;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../middleware/mod.rs"]
mod middleware;
#[path = "../routes/mod.rs"]
mod routes;

fn percentile(values: &[f64], percentile: f64) -> f64 {
    let index = ((values.len() as f64 * percentile).ceil() as usize).saturating_sub(1);
    values[index]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::from_file()?;
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, &config.database.url)?;
    let redis = RedisClient::open(config.webapp.redisurl.clone())?;

    let app = Router::new()
        .merge(routes::setup_routes())
        .layer(ServiceBuilder::new()
            .layer(Extension(redis))
            .layer(Extension(config.clone()))
            .layer(Extension(rb)));
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .expect("create test server")
            .serve(app.into_make_service())
            .await
            .expect("run test server");
    });

    let base = format!("http://{address}");
    let client = Client::new();
    tokio::time::sleep(Duration::from_millis(50)).await;

    let unauthenticated_status = client.get(format!("{base}/get")).send().await?.status();

    let invalid_login = client
        .post(format!("{base}/login"))
        .json(&json!({"username": "wrong", "password": "wrong"}))
        .send()
        .await?;
    assert_eq!(invalid_login.status(), StatusCode::UNAUTHORIZED);
    let invalid_login_status = invalid_login.status();
    let invalid_login_body = invalid_login.text().await?;
    assert!(invalid_login_body.contains("Invalid username or password"));

    let login = client
        .post(format!("{base}/login"))
        .json(&json!({"username": config.auth.username, "password": config.auth.password}))
        .send()
        .await?;
    assert_eq!(login.status(), StatusCode::OK);
    let valid_login_status = login.status();
    let login_body = login.text().await?;
    let session_id = login_body
        .strip_prefix("Logged in with session ID: ")
        .expect("login returns session_id")
        .to_owned();
    let auth = format!("Bearer {session_id}");

    let get_body = client.get(format!("{base}/get")).header("Authorization", &auth).send().await?.text().await?;
    assert!(get_body.contains("Hello, World!"));
    assert!(!get_body.contains("script"));

    let sanitized_body = client
        .post(format!("{base}/getBody"))
        .header("Authorization", &auth)
        .json(&json!({"html_code": "<h1>safe</h1><script>alert(1)</script>"}))
        .send()
        .await?
        .text()
        .await?;
    assert!(sanitized_body.contains("safe"));
    assert!(!sanitized_body.contains("script"));

    let form_response = client
        .post(format!("{base}/post"))
        .header("Authorization", &auth)
        .form(&[("custom_username", "tester"), ("custom_password", "secret")])
        .send()
        .await?;
    assert_eq!(form_response.status(), StatusCode::OK);

    let paginate = client
        .post(format!("{base}/paginate"))
        .header("Authorization", &auth)
        .json(&json!({"sql": "SELECT id, number, status FROM orders ORDER BY id", "page": 1, "page_size": 2}))
        .send()
        .await?;
    assert_eq!(paginate.status(), StatusCode::OK);
    let paginate_json = paginate.json::<Value>().await?;
    assert_eq!(paginate_json["data"].as_array().map(Vec::len), Some(2));

    let parameterized = client
        .post(format!("{base}/paginate-with-params"))
        .header("Authorization", &auth)
        .json(&json!({"sql": "SELECT id FROM orders WHERE status > ?", "page": 1, "page_size": 2, "params": [0]}))
        .send()
        .await?;
    let parameterized_status = parameterized.status();
    let parameterized_json = parameterized.json::<Value>().await?;
    assert_eq!(parameterized_status, StatusCode::OK);
    assert_eq!(parameterized_json["data"].as_array().map(Vec::len), Some(2));
    assert_eq!(parameterized_json["total"].as_u64(), Some(7));

    let mut tasks = Vec::new();
    let started = Instant::now();
    for _ in 0..10 {
        let client = client.clone();
        let auth = auth.clone();
        let endpoint = format!("{base}/get");
        tasks.push(tokio::spawn(async move {
            let mut timings = Vec::new();
            for _ in 0..20 {
                let request_started = Instant::now();
                let response = client.get(&endpoint).header("Authorization", &auth).send().await
                    .map_err(|error| error.to_string())?;
                if response.status() != StatusCode::OK {
                    return Err(format!("unexpected GET status: {}", response.status()));
                }
                timings.push(request_started.elapsed().as_secs_f64() * 1000.0);
            }
            Ok::<_, String>(timings)
        }));
    }
    let mut timings = Vec::new();
    for task in tasks {
        timings.extend(task.await?.expect("benchmark request succeeded"));
    }
    let elapsed_seconds = started.elapsed().as_secs_f64();
    timings.sort_by(|a, b| a.partial_cmp(b).expect("finite timings"));

    let logout = client.post(format!("{base}/logout")).header("Authorization", &auth).send().await?;
    assert_eq!(logout.status(), StatusCode::OK);
    let after_logout_status = client.get(format!("{base}/get")).header("Authorization", &auth).send().await?.status();

    println!("{}", json!({
        "database": "SQLite test.db",
        "functional": {
            "unauthenticated_get": unauthenticated_status.as_u16(),
            "invalid_login": {
                "status": invalid_login_status.as_u16(),
                "rejected": true
            },
            "valid_login": valid_login_status.as_u16(),
            "xss_sanitization": "passed",
            "form_post": 200,
            "pagination_rows": paginate_json["data"].as_array().map(Vec::len),
            "pagination_total": paginate_json["total"],
            "parameterized_pagination": {
                "status": parameterized_status.as_u16(),
                "rows": parameterized_json["data"].as_array().map(Vec::len),
                "total": parameterized_json["total"]
            },
            "logout": 200,
            "after_logout": after_logout_status.as_u16()
        },
        "performance": {
            "requests": timings.len(),
            "concurrency": 10,
            "elapsed_seconds": elapsed_seconds,
            "throughput_rps": timings.len() as f64 / elapsed_seconds,
            "latency_ms": {
                "min": timings[0],
                "p50": percentile(&timings, 0.50),
                "p95": percentile(&timings, 0.95),
                "max": timings[timings.len() - 1]
            }
        }
    }));
    Ok(())
}
