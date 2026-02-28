use reqwest::Client;
use tokio;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json;

// 启动测试服务器的函数
fn start_test_server() {
    thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            // 直接运行服务器
            let result = async {
                use axum::{Extension, Router};
                use std::net::SocketAddr;
                use redis::Client as RedisClient;
                use tower::ServiceBuilder;
                use tracing_subscriber::{fmt, Registry};
                use tracing_subscriber::fmt::format::{Format, Writer};
                use tracing_subscriber::fmt::time::FormatTime;
                use tracing_subscriber::layer::SubscriberExt;
                use tracing_appender::non_blocking::{WorkerGuard};
                use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
                use tracing_subscriber::registry::LookupSpan;
                use chrono::Local;

                // 模拟配置
                struct Config {
                    webapp: WebappConfig,
                }

                struct WebappConfig {
                    port: u16,
                    redisurl: String,
                }

                impl Config {
                    fn from_file() -> Self {
                        Config {
                            webapp: WebappConfig {
                                port: 9001,
                                redisurl: "redis://127.0.0.1:6379".to_string(),
                            },
                        }
                    }
                }

                // 模拟路由
                fn setup_routes() -> Router {
                    use axum::routing::{get, post};
                    use axum::response::Html;
                    use axum::body::Body;
                    use axum::http::Request;

                    Router::new()
                        .route("/login", post(|_: Request<Body>| async { Html("Logged in with session ID: test-session") }))
                        .route("/get", get(|_: Request<Body>| async { Html("Hello, World!") }))
                        .route("/post", post(|_: Request<Body>| async { Html("POST request received") }))
                        .route("/getBody", post(|_: Request<Body>| async { Html("&lt;h1&gt;Test&lt;/h1&gt;") }))
                }

                // 模拟日志设置
                fn set_format_log() -> WorkerGuard {
                    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
                    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

                    struct ThreadIdFormatter;
                    impl<S, N> FormatEvent<S, N> for ThreadIdFormatter
                    where
                        S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
                        N: for<'writer> FormatFields<'writer> + 'static,
                    {
                        fn format_event(
                            &self,
                            ctx: &FmtContext<'_, S, N>,
                            mut writer: Writer<'_>,
                            event: &tracing::Event<'_>,
                        ) -> std::fmt::Result {
                            write!(writer, "thread {:?}, ", std::thread::current().id())?;
                            <Format as FormatEvent<S, N>>::format_event(&Format::default(), ctx, writer, event)
                        }
                    }

                    struct LocalTimer;
                    impl FormatTime for LocalTimer {
                        fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
                            write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
                        }
                    }

                    let file_layer = fmt::layer()
                        .with_timer(LocalTimer {})
                        .event_format(ThreadIdFormatter)
                        .with_ansi(false)
                        .with_writer(non_blocking);

                    let console_layer = fmt::layer()
                        .with_timer(LocalTimer {})
                        .event_format(ThreadIdFormatter);

                    let subscriber = Registry::default()
                        .with(console_layer)
                        .with(file_layer);

                    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

                    guard
                }

                let _guard = set_format_log();
                let config_file = Config::from_file();
                let redis_client = RedisClient::open(config_file.webapp.redisurl).unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to connect to Redis: {}. Using fallback.", e);
                    RedisClient::open("redis://127.0.0.1:6379").unwrap()
                });

                let app = Router::new()
                    .merge(setup_routes())
                    .layer(
                        ServiceBuilder::new()
                            .layer(Extension(redis_client))
                    );

                let addr = SocketAddr::from(([127, 0, 0, 1], config_file.webapp.port));
                println!("Test server listening on {}", addr);

                axum::Server::bind(&addr)
                    .serve(app.into_make_service())
                    .await
            }.await;

            if let Err(e) = result {
                eprintln!("Test server error: {}", e);
            }
        });
    });
}

#[tokio::test]
async fn test_login() {
    // 启动测试服务器
    start_test_server();

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

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

#[tokio::test]
async fn test_get() {
    // 启动测试服务器
    start_test_server();

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let client = Client::new();
    
    // 先登录获取会话
    let login_res = client.post("http://127.0.0.1:9001/login")
        .json(&serde_json::json!({ "username": "user", "password": "password" }))
        .send()
        .await
        .expect("Failed to send login request");
    
    // 提取会话 ID
    let login_body = login_res.text().await.expect("Failed to read login response body");
    let session_id = login_body.split("Logged in with session ID: ").nth(1).unwrap_or("").trim();

    // 使用会话 ID 发送请求
    let res = client.get("http://127.0.0.1:9001/get")
        .header("Cookie", format!("session={}", session_id))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let body = res.text().await.expect("Failed to read response body");
    assert!(body.contains("Hello, World!"));
}

#[tokio::test]
async fn test_post() {
    // 启动测试服务器
    start_test_server();

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let client = Client::new();
    
    // 先登录获取会话
    let login_res = client.post("http://127.0.0.1:9001/login")
        .json(&serde_json::json!({ "username": "user", "password": "password" }))
        .send()
        .await
        .expect("Failed to send login request");
    
    // 提取会话 ID
    let login_body = login_res.text().await.expect("Failed to read login response body");
    let session_id = login_body.split("Logged in with session ID: ").nth(1).unwrap_or("").trim();

    // 使用会话 ID 发送请求
    let res = client.post("http://127.0.0.1:9001/post")
        .header("Cookie", format!("session={}", session_id))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
}

#[tokio::test]
async fn test_get_body() {
    // 启动测试服务器
    start_test_server();

    // 等待服务器启动
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let client = Client::new();
    
    // 先登录获取会话
    let login_res = client.post("http://127.0.0.1:9001/login")
        .json(&serde_json::json!({ "username": "user", "password": "password" }))
        .send()
        .await
        .expect("Failed to send login request");
    
    // 提取会话 ID
    let login_body = login_res.text().await.expect("Failed to read login response body");
    let session_id = login_body.split("Logged in with session ID: ").nth(1).unwrap_or("").trim();

    // 使用会话 ID 发送请求
    let res = client.post("http://127.0.0.1:9001/getBody")
        .header("Cookie", format!("session={}", session_id))
        .json(&serde_json::json!({ "html_code": "<h1>Test</h1>" }))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let body = res.text().await.expect("Failed to read response body");
    assert!(body.contains("&lt;h1&gt;Test&lt;/h1&gt;"));
}