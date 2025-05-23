use crate::handlers::item_handler::list_items;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/items", get(list_items))
}
