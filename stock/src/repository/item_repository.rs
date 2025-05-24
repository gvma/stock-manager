use crate::domain::item::Item;
use sqlx::PgPool;

pub struct ItemRepository;

impl ItemRepository {
    pub async fn listar(pool: &PgPool) -> Result<Vec<Item>, sqlx::Error> {
        let itens = sqlx::query_as!(Item, "SELECT id, nome FROM items")
            .fetch_all(pool)
            .await?;
        Ok(itens)
    }
}
