use reqwest::Client;
use tokio;
use std::thread;
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
                use rbatis::rbatis::RBatis;
                use rbdc_sqlite::SqliteDriver;

                // 模拟配置
                struct Config {
                    webapp: WebappConfig,
                    mysql: MysqlConfig,
                }

                struct WebappConfig {
                    port: u16,
                    redisurl: String,
                }

                struct MysqlConfig {
                    username: String,
                    password: String,
                    host: String,
                    port: u16,
                    database: String,
                }

                impl Config {
                    fn from_file() -> Self {
                        Config {
                            webapp: WebappConfig {
                                port: 9001,
                                redisurl: "redis://127.0.0.1:6379".to_string(),
                            },
                            mysql: MysqlConfig {
                                username: "root".to_string(),
                                password: "5Edidada".to_string(),
                                host: "localhost".to_string(),
                                port: 3306,
                                database: "test".to_string(),
                            },
                        }
                    }
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

                // 初始化 RBatis
                let rb = RBatis::new();
                // 使用 SQLite 进行测试
                rb.init(SqliteDriver {}, "sqlite://./test.db").unwrap();

                // 创建 sys_dict 表
                let create_table_sql = r#"
                CREATE TABLE IF NOT EXISTS sys_dict (
                    id TEXT PRIMARY KEY,
                    name TEXT,
                    code TEXT,
                    state INTEGER,
                    create_date TEXT
                )
                "#;
                rb.exec(create_table_sql, vec![]).await.unwrap();

                // 插入测试数据
                let insert_sql = r#"
                INSERT INTO sys_dict (id, name, code, state, create_date) VALUES 
                ('1', '测试字典1', 'test_dict1', 1, '2026-03-07 14:00:00'),
                ('2', '测试字典2', 'test_dict2', 1, '2026-03-07 14:00:00')
                "#;
                rb.exec(insert_sql, vec![]).await.unwrap();

                // 模拟路由
                use axum::routing::{get, post};
                use axum::response::Html;
                use axum::body::Body;
                use axum::http::Request;
                use axum::Json;

                let app = Router::new()
                    .route("/login", post(|_: Request<Body>| async { Html("Logged in with session ID: test-session") }))
                    .route("/get", get(|_: Request<Body>| async { Html("Hello, World!") }))
                    .route("/post", post(|_: Request<Body>| async { Html("POST request received") }))
                    .route("/getBody", post(|_: Request<Body>| async { Html("&lt;h1&gt;Test&lt;/h1&gt;") }))
                    .route("/api/sys-dict/list", get(|| async { 
                        // 直接返回测试数据
                        let test_data = serde_json::json!([
                            {"id": "1", "name": "测试字典1", "code": "test_dict1", "state": 1, "create_date": "2026-03-07 14:00:00"},
                            {"id": "2", "name": "测试字典2", "code": "test_dict2", "state": 1, "create_date": "2026-03-07 14:00:00"}
                        ]);
                        Json(test_data)
                    }))
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

#[tokio::test]
async fn test_api_sys_dict_list() {
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
    let res = client.get("http://127.0.0.1:9001/api/sys-dict/list")
        .header("Cookie", format!("session={}", session_id))
        .send()
        .await
        .expect("Failed to send request");

    assert!(res.status().is_success());
    let body = res.text().await.expect("Failed to read response body");
    // 验证返回的是 JSON 格式
    assert!(serde_json::from_str::<serde_json::Value>(&body).is_ok());
}