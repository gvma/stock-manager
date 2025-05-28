use lapin::{Channel, Connection, ConnectionProperties};
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn start_db_pool() -> sqlx::PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Erro ao conectar no banco")
}

pub async fn start_event_queue() -> Channel {
    let rabbitmq_url = env::var("RABBITMQ_URL").expect("RABBITMQ_URL não definido");    

    Connection::connect(&rabbitmq_url, ConnectionProperties::default())
        .await
        .expect("Erro ao conectar com o sistema de mensagens")
        .create_channel()
        .await
        .expect("Erro ao criar canal de mensagens")
}