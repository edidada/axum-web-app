use std::error::Error;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::thread;
use chrono::Local;
use tracing_subscriber::{fmt, Registry};
use tracing_subscriber::fmt::format::{Format, Writer};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use redis::Client;
use tower::ServiceBuilder;
use tracing_appender::non_blocking::{WorkerGuard};
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use crate::config::Config;
use rbatis::rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_sqlite::driver::SqliteDriver;

mod handlers;
mod routes;
mod middleware;
mod config;
mod domain;

fn init_rbatis(config: &Config) -> Result<RBatis, Box<dyn Error>> {
    let rb = RBatis::new();
    match config.database.db_type.as_str() {
        "sqlite" => rb.init(SqliteDriver {}, &config.database.url)?,
        "mysql" => rb.init(MysqlDriver {}, &config.database.url)?,
        other => return Err(format!("unsupported database type: {other}").into()),
    }
    Ok(rb)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _guard = set_format_log();
    let config_file = Config::from_file().map_err(|error| {
        tracing::error!("Failed to load config/default.toml: {error}");
        error
    })?;
    let rb = init_rbatis(&config_file)?;
    let redis_client = Client::open(config_file.webapp.redisurl.clone())?;

    let app = Router::new()
        .merge(routes::setup_routes())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(redis_client))
                .layer(Extension(config_file.clone()))
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
    // 初始化 log 到 tracing 的桥接
    tracing_log::LogTracer::init().expect("Failed to init LogTracer");
    
    // 创建一个滚动文件追加器
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 创建一个日志记录器层，将日志输出到文件
    let file_layer = fmt::layer()
        .with_timer(LocalTimer {}) // 传递 LocalTimer 实例
        .event_format(ThreadIdFormatter)
        .with_ansi(false) // 禁用 ANSI 样式
        .with_writer(non_blocking);

    // 创建一个格式化层，将日志输出到控制台
    let console_layer = fmt::layer()
        .with_timer(LocalTimer {}) // 传递 LocalTimer 实例
        .event_format(ThreadIdFormatter);

    // 创建一个订阅者，将文件层和控制台层添加到订阅者中
    let subscriber = Registry::default()
        .with(console_layer)
        .with(file_layer);

    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    guard
}

// 自定义日志格式化结构体
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
        // 获取当前线程 ID
        let thread_id = thread::current().id();
        // 写入线程 ID 信息
        write!(writer, "thread {:?}, ", thread_id)?;

        // 调用默认的格式化方法处理事件
        <Format as FormatEvent<S, N>>::format_event(&Format::default(), ctx, writer, event)
    }
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}
