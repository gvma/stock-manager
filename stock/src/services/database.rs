use std::env;

use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, ConnectOptions, PgPool};

use super::Connect;

pub struct Database;

impl Connect<PgPool> for Database {
    async fn connect() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");
    
        let connection_options =
            PgConnectOptions::from_url(&database_url.parse().expect("DATABASE_URL inválido"))
                .expect("DATABASE_URL inválido")
                .database("stock-manager-stock");
    
        PgPoolOptions::new()
            .max_connections(5)
            .connect_with(connection_options)
            .await
            .expect("Erro ao conectar no banco")
    }
}
