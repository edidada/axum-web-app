// This file is the entry point of the application. It sets up the Axum server, initializes routes, and starts the application.

use axum::{Router, routing::get, routing::post};
use std::net::SocketAddr;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::routes::setup_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}