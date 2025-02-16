// This file is the entry point of the application. It sets up the Axum server, initializes routes, and starts the application.

use axum::{Router, routing::get, routing::post};
use std::net::SocketAddr;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    // 创建一个按天滚动的日志文件追加器
    let appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    // 创建一个日志记录器层，将日志输出到文件
    let file_layer = fmt::layer().with_writer(non_blocking);

    // 创建一个基于环境变量的过滤器层
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 创建一个订阅器，将过滤器层和文件层组合起来
    let subscriber = Registry::default()
        .with(env_filter)
        .with(file_layer);
    // 设置全局订阅器
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let app = Router::new()
        .merge(routes::routes::setup_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}