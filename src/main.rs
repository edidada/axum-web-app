use std::{error::Error, fs, net::SocketAddr, thread};

use axum::{Extension, Router};
use chrono::Local;
use once_cell::sync::OnceCell;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;
use rbatis::rbatis::RBatis;
use redis::Client;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;

use crate::config::Config;
use crate::handlers::health_handler::init_start_time;

mod handlers;
mod routes;
mod middleware;
mod config;
mod domain;
#[cfg(feature = "minio")]
mod infrastructure;

static RB: OnceCell<RBatis> = OnceCell::new();

fn init_rbatis(config: &Config) -> Result<(), Box<dyn Error>> {
    let rb = RBatis::new();
    let database_url = config.get_database_url().map_err(|error| format!("database configuration error: {error}"))?;
    match config.database.db_type.as_str() {
        "sqlite" => rb.init(SqliteDriver {}, &database_url)?,
        "mysql" => rb.init(MysqlDriver {}, &database_url)?,
        "postgresql" | "postgres" | "pg" => rb.init(PgDriver {}, &database_url)?,
        other => return Err(format!("unsupported database type: {other}").into()),
    }
    RB.set(rb).map_err(|_| "RBatis is already initialized")?;
    Ok(())
}

fn get_rbatis() -> &'static RBatis {
    RB.get().expect("RBatis is not initialized")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_file = Config::from_file()?;
    let _guard = set_format_log();
    init_start_time();
    init_rbatis(&config_file)?;

    #[cfg(feature = "minio")]
    if let Err(error) = infrastructure::minio_client::init_minio_client().await {
        tracing::warn!("Failed to initialize MinIO client: {error}");
    }

    let redis_client = Client::open(config_file.webapp.redisurl.clone())?;
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
                .layer(Extension(config_file.clone()))
                .layer(Extension(get_rbatis()))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], config_file.webapp.port));
    tracing::info!("thread {:?}, listening on {} using {} profile", thread::current().id(), addr, config_file.database.db_type);
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

fn set_format_log() -> WorkerGuard {
    fs::create_dir_all("logs").expect("create logs directory");
    tracing_log::LogTracer::init().expect("Failed to init LogTracer");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    let file_appender = tracing_appender::rolling::daily("logs", "app");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().with_timer(LocalTimer {}).with_ansi(false).with_writer(non_blocking);
    let console_layer = fmt::layer().with_timer(LocalTimer {});
    tracing::subscriber::set_global_default(Registry::default().with(env_filter).with(console_layer).with(file_layer))
        .expect("Failed to set subscriber");
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
        let results = Orders::select_order_with_details(&rb, Some("order_001"), None).await;

        match results {
            Ok(rows) => {
                tracing::info!("查询成功，返回 {} 行数据", rows.len());
                for row in &rows {
                    tracing::info!("订单号: {:?}, 明细名称: {:?}", row.number, row.name);
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
