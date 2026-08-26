use axum::{Extension, Router};
use rbatis::rbatis::RBatis;
use rbdc_sqlite::driver::SqliteDriver;
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use std::{
    error::Error,
    time::{Duration, Instant},
};
use tower::ServiceBuilder;
use uuid::Uuid;

use axum_web_app::{auth_session, config, jwt, middleware, routes};

fn percentile(values: &[f64], percentile: f64) -> f64 {
    let index = ((values.len() as f64 * percentile).ceil() as usize).saturating_sub(1);
    values[index]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("APP_PROFILE", "sqlite");
    let config = config::Config::from_file()?;
    let rb = RBatis::new();
    let sqlite_path = std::env::temp_dir().join(format!("axum-security-{}.db", Uuid::new_v4()));
    let sqlite_url = format!("sqlite://{}", sqlite_path.display());
    rb.init(SqliteDriver {}, &sqlite_url)?;
    let rb: &'static RBatis = Box::leak(Box::new(rb));
    auth_session::init_store(rb).await?;
    rb.exec(
        "CREATE TABLE orders (id TEXT PRIMARY KEY, number TEXT, status INTEGER)",
        vec![],
    )
    .await?;
    rb.exec(
        "INSERT INTO orders (id, number, status) VALUES ('1','O1',1),('2','O2',1),('3','O3',1),('4','O4',1),('5','O5',1),('6','O6',1),('7','O7',1)",
        vec![],
    )
    .await?;

    let app = Router::new().merge(routes::setup_routes()).layer(
        ServiceBuilder::new()
            .layer(axum::middleware::from_fn(
                middleware::security_headers::add_security_headers,
            ))
            .layer(Extension(config.clone()))
            .layer(Extension(rb)),
    );
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let server = tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .expect("create test server")
            .serve(app.into_make_service())
            .await
            .expect("run test server");
    });

    let base = format!("http://{address}");
    // CI/开发机可能配置系统 HTTP 代理；回环测试必须直连本地临时端口。
    let client = Client::builder().no_proxy().build()?;
    tokio::time::sleep(Duration::from_millis(50)).await;

    let unauthenticated = client.get(format!("{base}/get")).send().await?;
    let unauthenticated_status = unauthenticated.status();
    assert!(unauthenticated
        .headers()
        .contains_key("content-security-policy"));

    let invalid_login = client
        .post(format!("{base}/api/login"))
        .json(&json!({"username": "wrong", "password": "wrong"}))
        .send()
        .await?;
    let invalid_login_status = invalid_login.status();
    let invalid_login_body = invalid_login.text().await?;
    assert_eq!(
        invalid_login_status,
        StatusCode::UNAUTHORIZED,
        "invalid login response: {invalid_login_body}"
    );
    assert!(invalid_login_body.contains("用户名或密码错误"));

    let hostile_login = client
        .post(format!("{base}/api/login"))
        .header("Origin", "https://attacker.example")
        .json(&json!({"username": config.auth.username, "password": config.auth.password}))
        .send()
        .await?;
    assert_eq!(hostile_login.status(), StatusCode::FORBIDDEN);

    let login = client
        .post(format!("{base}/api/login"))
        .json(&json!({"username": config.auth.username, "password": config.auth.password}))
        .send()
        .await?;
    assert_eq!(login.status(), StatusCode::OK);
    let valid_login_status = login.status();
    let raw_set_cookies = login
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let set_cookies = raw_set_cookies
        .iter()
        .map(|value| value.split(';').next().unwrap_or_default().to_owned())
        .collect::<Vec<_>>();
    let cookie_header = set_cookies.join("; ");
    let csrf_token = set_cookies
        .iter()
        .find_map(|cookie| cookie.strip_prefix("csrf_token="))
        .expect("login sets csrf_token cookie")
        .to_owned();
    assert!(set_cookies
        .iter()
        .any(|cookie| cookie.starts_with("access_token=")));
    let access_cookie = raw_set_cookies
        .iter()
        .find(|cookie| cookie.starts_with("access_token="))
        .expect("access cookie exists");
    assert!(access_cookie.contains("HttpOnly"));
    assert!(access_cookie.contains("SameSite=Lax"));
    let login_json = login.json::<Value>().await?;
    assert!(
        login_json.get("access_token").is_none(),
        "JWT must not be exposed to browser JavaScript"
    );
    assert_eq!(login_json["role"], "admin");

    let get_body = client
        .get(format!("{base}/get"))
        .header("Cookie", &cookie_header)
        .send()
        .await?
        .text()
        .await?;
    assert!(get_body.contains("Hello, World!"));
    assert!(!get_body.contains("script"));

    let sanitized_body = client
        .post(format!("{base}/getBody"))
        .header("Cookie", &cookie_header)
        .header("X-CSRF-Token", &csrf_token)
        .json(&json!({"html_code": "<h1>safe</h1><script>alert(1)</script>"}))
        .send()
        .await?
        .text()
        .await?;
    assert!(sanitized_body.contains("safe"));
    assert!(!sanitized_body.contains("script"));

    let form_response = client
        .post(format!("{base}/post"))
        .header("Cookie", &cookie_header)
        .header("X-CSRF-Token", &csrf_token)
        .form(&[("custom_username", "tester"), ("custom_password", "secret")])
        .send()
        .await?;
    assert_eq!(form_response.status(), StatusCode::OK);

    let paginate = client
        .post(format!("{base}/paginate"))
        .header("Cookie", &cookie_header)
        .header("X-CSRF-Token", &csrf_token)
        .json(&json!({"sql": "SELECT id, number, status FROM orders ORDER BY id", "page": 1, "page_size": 2}))
        .send()
        .await?;
    assert_eq!(paginate.status(), StatusCode::OK);
    let paginate_json = paginate.json::<Value>().await?;
    assert_eq!(paginate_json["data"].as_array().map(Vec::len), Some(2));

    let parameterized = client
        .post(format!("{base}/paginate-with-params"))
        .header("Cookie", &cookie_header)
        .header("X-CSRF-Token", &csrf_token)
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
        let cookie_header = cookie_header.clone();
        let endpoint = format!("{base}/get");
        tasks.push(tokio::spawn(async move {
            let mut timings = Vec::new();
            for _ in 0..20 {
                let request_started = Instant::now();
                let response = client
                    .get(&endpoint)
                    .header("Cookie", &cookie_header)
                    .send()
                    .await
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

    let missing_csrf = client
        .post(format!("{base}/api/logout"))
        .header("Cookie", &cookie_header)
        .send()
        .await?;
    assert_eq!(missing_csrf.status(), StatusCode::FORBIDDEN);

    let hostile_origin = client
        .post(format!("{base}/api/logout"))
        .header("Cookie", &cookie_header)
        .header("X-CSRF-Token", &csrf_token)
        .header("Origin", "https://attacker.example")
        .send()
        .await?;
    assert_eq!(hostile_origin.status(), StatusCode::FORBIDDEN);

    let (viewer_token, viewer_claims) = jwt::issue("readonly", "viewer", &config.jwt)?;
    auth_session::create(rb, &viewer_claims, "api-client-csrf-not-used").await?;
    let viewer_forbidden = client
        .post(format!("{base}/category/delete"))
        .header("Authorization", format!("Bearer {viewer_token}"))
        .json(&json!("nonexistent"))
        .send()
        .await?;
    assert_eq!(viewer_forbidden.status(), StatusCode::FORBIDDEN);
    let viewer_read = client
        .get(format!("{base}/get"))
        .header("Authorization", format!("Bearer {viewer_token}"))
        .send()
        .await?;
    assert_eq!(viewer_read.status(), StatusCode::OK);

    let logout = client
        .post(format!("{base}/api/logout"))
        .header("Cookie", &cookie_header)
        .header("X-CSRF-Token", &csrf_token)
        .header("Origin", "http://localhost:5173")
        .send()
        .await?;
    assert_eq!(logout.status(), StatusCode::OK);
    let after_logout_status = client
        .get(format!("{base}/get"))
        .header("Cookie", &cookie_header)
        .send()
        .await?
        .status();

    println!(
        "{}",
        json!({
            "database": "isolated temporary SQLite",
            "functional": {
                "unauthenticated_get": unauthenticated_status.as_u16(),
                "invalid_login": {
                    "status": invalid_login_status.as_u16(),
                    "rejected": true
                },
                "valid_login": valid_login_status.as_u16(),
                "xss_sanitization": "passed",
                "csrf_missing": missing_csrf.status().as_u16(),
                "csrf_hostile_origin": hostile_origin.status().as_u16(),
                "hostile_login_origin": hostile_login.status().as_u16(),
                "viewer_write": viewer_forbidden.status().as_u16(),
                "viewer_read": viewer_read.status().as_u16(),
                "csp_header": "present",
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
        })
    );
    server.abort();
    Ok(())
}
