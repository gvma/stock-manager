mod event;

use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(event::routes())
}
