use crate::domain::item::Item;
use crate::repository::item_repository::ItemRepository;
use axum::{Extension, Json};
use sqlx::PgPool;

pub async fn list_items(Extension(pool): Extension<PgPool>) -> Json<Vec<Item>> {
    let items = ItemRepository::list(&pool).await.unwrap_or_default();
    Json(items)
}
