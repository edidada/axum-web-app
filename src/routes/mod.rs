use axum::{routing::get, routing::post, Router};
use crate::handlers::get_handler::{get_handler, get_body_handler};
use crate::handlers::post_handler::post_handler;
use crate::handlers::login_handler::login_handler;
use crate::handlers::logout_handler::logout_handler;
use crate::middleware::session_validation::validate_session;

pub fn setup_routes() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/get", get(get_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/post", post(post_handler).layer(axum::middleware::from_fn(validate_session)))
        .route("/getBody", post(get_body_handler).layer(axum::middleware::from_fn(validate_session)))
}