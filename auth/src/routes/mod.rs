pub mod user;

use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(user::routes())
}