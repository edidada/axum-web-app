// This file is the entry point of the application. It sets up the Axum server, initializes routes, and starts the application.

use axum::{Extension, Router};
use std::net::SocketAddr;
use std::thread;
use chrono::Local;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::fmt::format::{Format, Writer};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use redis::Client;
use tower::ServiceBuilder;
use tracing_appender::non_blocking;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;
use crate::config::Config;

mod handlers;
mod routes;
mod middleware;
mod config;

#[tokio::main]
async fn main() {
    setFormatLog();

    let configFile = Config::from_file();
    // Initialize Redis client
    let redis_client = Client::open(configFile.webapp.redisurl).unwrap();

    let app = Router::new()
        .merge(routes::setup_routes())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(redis_client))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], configFile.webapp.port));
    tracing::info!("thread {:?},Listening on {}", thread::current().id(), addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn setFormatLog() {
    // 创建一个滚动文件追加器，这里简单示例为非滚动的文件写入
    let file_appender = std::fs::File::create("app.log").expect("Failed to create log file");
    let (non_blocking, _guard) = non_blocking(file_appender);

    // 创建一个日志记录器层，将日志输出到文件
    let file_layer = fmt::layer()
        .event_format(ThreadIdFormatter)
        .with_timer(LocalTimer {}) // 传递 LocalTimer 实例
        .with_ansi(false) // 禁用 ANSI 样式
        .with_writer(non_blocking);

    // 创建一个格式化层，将日志输出到控制台
    let console_layer = fmt::layer()
        .event_format(ThreadIdFormatter)
        .with_timer(LocalTimer {}) // 传递 LocalTimer 实例
        .pretty();

    // 创建一个订阅者，将文件层和控制台层添加到订阅者中
    let subscriber = Registry::default()
        .with(console_layer)
        .with(file_layer);

    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // // 创建一个按天滚动的日志文件追加器
    // let appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    // // 创建一个日志记录器层，将日志输出到文件
    // let file_layer = fmt::layer().with_timer(LocalTimer)
    // .with_ansi(false) // 禁用 ANSI 样式
    // .with_writer(non_blocking);

    // // 创建一个基于环境变量的过滤器层
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // // 创建一个订阅器，将过滤器层和文件层组合起来
    // let subscriber = Registry::default()
    // .with(env_filter)
    // .with(file_layer);
    // // 设置全局订阅器
    // tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
fn setup_logging() {
    // 创建自定义日志层
    let custom_layer = fmt::layer().event_format(ThreadIdFormatter);

    // 创建订阅者并添加自定义层
    let subscriber = Registry::default().with(custom_layer);

    // 设置全局订阅者
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
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