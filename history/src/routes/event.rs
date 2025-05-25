use crate::handlers::event_handler::list;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/events", get(list))
}
