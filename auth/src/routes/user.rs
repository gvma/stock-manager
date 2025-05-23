use axum::{routing::post, Router};
use crate::handlers::user_handler::{sign_in, sign_up};

pub fn routes() -> Router {
    Router::new()
        .route("/sign_in", post(sign_in))
        .route("/sign_up", post(sign_up))
}