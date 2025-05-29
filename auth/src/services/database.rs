use std::env;

use sqlx::{postgres::{PgPoolOptions}, PgPool};

use super::Connect;

pub struct Database;

impl Connect<PgPool> for Database {
    async fn connect() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");
    
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Erro ao conectar no banco")
    }
}
