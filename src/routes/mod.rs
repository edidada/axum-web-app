pub mod routes {
    use axum::{routing::get, routing::post, Router};
    use crate::handlers::get_handler::{get_handler, get_body_handler};
    use crate::handlers::post_handler::post_handler;

    pub fn setup_routes() -> Router {
        Router::new()
            .route("/get", get(get_handler))
            .route("/post", post(post_handler))
            .route("/getBody", post(get_body_handler))

    }
}