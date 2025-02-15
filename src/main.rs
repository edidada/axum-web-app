// This file is the entry point of the application. It sets up the Axum server, initializes routes, and starts the application.

use axum::{Router, routing::get, routing::post};
use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::writer::MakeWriterExt;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    // Initialize tracing
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking.and(std::io::stdout))
        .init();

    let app = Router::new()
        .merge(routes::routes::setup_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}