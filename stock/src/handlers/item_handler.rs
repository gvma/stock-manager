use crate::domain::Item;
use crate::repository::ItemRepository;
use axum::{Extension, Json};
use sqlx::PgPool;

pub async fn list_items(Extension(pool): Extension<PgPool>) -> Json<Vec<Item>> {
    let items = ItemRepository::list(&pool).await.unwrap();
    Json(items)
}
