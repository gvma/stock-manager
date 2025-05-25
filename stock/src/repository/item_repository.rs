use crate::domain::Item;
use sqlx::PgPool;

pub struct ItemRepository;

impl ItemRepository {
    pub async fn list(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
        let items = sqlx::query_as!(Item, "SELECT * FROM items ORDER BY name")
            .fetch_all(pool)
            .await?;
        Ok(items)
    }
}
