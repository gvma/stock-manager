use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn start_db_pool() -> sqlx::PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Erro ao conectar no banco")
}
