pub mod item;

use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(item::routes())
}
