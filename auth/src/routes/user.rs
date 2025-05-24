use axum::{routing::post, Router};
use crate::handlers::user_handler::{request_password_reset_code, reset_password, sign_in, sign_up};

pub fn routes() -> Router {
    Router::new()
        .route("/sign_in", post(sign_in))
        .route("/sign_up", post(sign_up))
        .route("/request_password_reset_code", post(request_password_reset_code))
        .route("/reset_password", post(reset_password))
}