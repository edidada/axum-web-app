use std::error::Error;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::{fs, thread};
use chrono::Local;
use tracing_subscriber::{fmt, Registry, EnvFilter};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use redis::Client;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_appender::non_blocking::{WorkerGuard};
use crate::config::Config;
use rbatis::rbatis::RBatis;
use crate::handlers::health_handler::init_start_time;

// 导入数据库驱动
use rbdc_pg::driver::PgDriver;

mod handlers;
mod routes;
mod middleware;
mod config;
mod domain;

// 定义静态的 RBatis 实例
static mut RB: Option<RBatis> = None;

// 异步初始化函数 - 支持 MySQL 和 PostgreSQL
async fn init_rbatis() -> Result<(), Box<dyn Error>> {
    let config = Config::from_file();
    let rb = RBatis::new();
    
    // 直接使用 PostgreSQL 配置
    let db_url = "postgres://ibqo:@127.0.0.1:5432/axum_db";
    tracing::info!("Initializing PostgreSQL connection to 127.0.0.1:5432");
    rb.init(PgDriver {}, db_url)?;
    
    unsafe {
        RB = Some(rb);
    }
    Ok(())
}

// 获取单例的函数
fn get_rbatis() -> &'static RBatis {
    unsafe {
        RB.as_ref().expect("RBatis is not initialized")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化启动时间（用于健康检查）
    init_start_time();
    
    // 初始化 RBatis
    init_rbatis().await?;

    let _guard = set_format_log();

    let config_file = Config::from_file();
    // Initialize Redis client
    let redis_client = Client::open(config_file.webapp.redisurl).unwrap_or_else(|e| {
        eprintln!("Warning: Failed to connect to Redis: {}. Using fallback.", e);
        // 使用本地 Redis 作为 fallback
        Client::open("redis://127.0.0.1:6379").unwrap()
    });

    // 获取 RBatis 实例
    let rb = get_rbatis();

    // 创建 CORS 中间件
    // 注意：当使用 credentials: 'include' 时，allow_origin 不能是 Any，必须指定具体的源
    // 同时，allow_headers 也不能是 Any，必须指定具体的头部
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE, axum::http::Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        .allow_credentials(true);

    let app = Router::new()
        .merge(routes::setup_routes())
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(axum::middleware::from_fn(middleware::request_logging::log_request))
                .layer(Extension(redis_client))
                .layer(Extension(rb))
        );

    // Change the port number here if necessary
    let addr = SocketAddr::from(([127, 0, 0, 1], config_file.webapp.port));
    tracing::info!("thread {:?},Listening on {}", thread::current().id(), addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn set_format_log() -> WorkerGuard {
    // Set up logging
    fs::create_dir_all("logs").unwrap();
    // 初始化 log 到 tracing 的桥接
    tracing_log::LogTracer::init().expect("Failed to init LogTracer");
    
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    // 创建一个滚动文件追加器
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 创建一个日志记录器层，将日志输出到文件
    let file_layer = fmt::layer()
        .with_timer(LocalTimer {}) // 传递 LocalTimer 实例
        .with_ansi(false) // 禁用 ANSI 样式
        .with_writer(non_blocking);

    // 创建一个格式化层，将日志输出到控制台
    let console_layer = fmt::layer()
        .with_timer(LocalTimer {}); // 传递 LocalTimer 实例

    // 创建一个订阅者，将文件层和控制台层添加到订阅者中
    let subscriber = Registry::default()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer);

    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    guard
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S%.6f"))
    }
}


#[cfg(test)]
mod tests {
    use rbatis::{PageRequest, RBatis};
    use rbdc_sqlite::SqliteDriver;
    use crate::domain::table::orders::Orders;

    // 初始化日志的辅助函数
    fn init_test_log() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
        // 初始化 log 到 tracing 的桥接
        let _ = tracing_log::LogTracer::init();
    }

    // 初始化 SQLite 用于测试
    async fn init_sqlite_rbatis() -> RBatis {
        let rb = RBatis::new();
        rb.init(SqliteDriver {}, "sqlite://:memory:").unwrap();
        rb
    }

    #[tokio::test]
    async fn test_select_order_with_details() -> Result<(), Box<dyn std::error::Error>> {
        init_test_log();

        let rb = init_sqlite_rbatis().await;

        // 创建 orders 表
        let create_orders_sql = r#"
            CREATE TABLE orders (
                id TEXT PRIMARY KEY,
                number TEXT,
                status INTEGER,
                user_id TEXT,
                address_book_id TEXT,
                order_time TEXT,
                checkout_time TEXT,
                pay_method INTEGER,
                pay_status INTEGER,
                amount REAL,
                remark TEXT,
                phone TEXT,
                address TEXT,
                user_name TEXT,
                consignee TEXT,
                cancel_reason TEXT,
                rejection_reason TEXT,
                cancel_time TEXT,
                estimated_delivery_time TEXT,
                delivery_status INTEGER,
                delivery_time TEXT,
                pack_amount INTEGER,
                tableware_number INTEGER,
                tableware_status INTEGER
            )
        "#;
        rb.exec(create_orders_sql, vec![]).await?;

        // 创建 order_detail 表
        let create_detail_sql = r#"
            CREATE TABLE order_detail (
                id TEXT PRIMARY KEY,
                name TEXT,
                image TEXT,
                order_id TEXT,
                dish_id TEXT,
                setmeal_id TEXT,
                dish_flavor TEXT,
                number INTEGER,
                amount REAL
            )
        "#;
        rb.exec(create_detail_sql, vec![]).await?;

        // 插入测试数据到 orders
        let insert_order_sql = r#"
            INSERT INTO orders (id, number, status, user_id, amount, consignee)
            VALUES (?, ?, ?, ?, ?, ?)
        "#;
        rb.exec(insert_order_sql, vec![
            rbs::to_value!("order_001"),
            rbs::to_value!("ORD20240307001"),
            rbs::to_value!(1),
            rbs::to_value!("user_001"),
            rbs::to_value!(99.99),
            rbs::to_value!("张三"),
        ]).await?;

        // 插入测试数据到 order_detail
        let insert_detail_sql = r#"
            INSERT INTO order_detail (id, name, order_id, dish_id, number, amount)
            VALUES (?, ?, ?, ?, ?, ?)
        "#;
        rb.exec(insert_detail_sql, vec![
            rbs::to_value!("detail_001"),
            rbs::to_value!("宫保鸡丁"),
            rbs::to_value!("order_001"),
            rbs::to_value!("dish_001"),
            rbs::to_value!(2),
            rbs::to_value!(49.99),
        ]).await?;

        rb.exec(insert_detail_sql, vec![
            rbs::to_value!("detail_002"),
            rbs::to_value!("麻婆豆腐"),
            rbs::to_value!("order_001"),
            rbs::to_value!("dish_002"),
            rbs::to_value!(1),
            rbs::to_value!(29.99),
        ]).await?;

        // 测试关联查询
        let results = Orders::select_order_with_details(&rb, "order_001").await;

        match results {
            Ok(rows) => {
                tracing::info!("查询成功，返回 {} 行数据", rows.len());
                for row in &rows {
                    tracing::info!("订单: {:?}, 明细: {:?}", row.order, row.detail);
                }
                assert!(!rows.is_empty(), "应该返回关联数据");
            }
            Err(e) => {
                tracing::error!("查询失败: {:?}", e);
                return Err(e.into());
            }
        }

        Ok(())
    }
}
