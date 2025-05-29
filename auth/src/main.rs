mod services;
mod routes;
mod repository;
mod domain;
mod handlers;

use lapin::{options::QueueDeclareOptions, types::FieldTable};
use routes::routes;
use tracing_subscriber;
use axum::{Extension};
use services::{Connect, Database, EventQueue};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // Inicializa os logs
    tracing_subscriber::fmt::init();
    
    dotenv().ok();

    let pool = Database::connect().await;
    let queue = EventQueue::connect().await;

    queue.queue_declare("stock-manager-auth", QueueDeclareOptions::default(), FieldTable::default()).await.unwrap();

    // Constrói as rotas do app com acesso ao DB nas rotas (extension e layer)
    let app = routes().layer(Extension(pool)).layer(Extension(queue));

    // Abre o socket
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4001")
        .await
        .expect("Erro ao bindar porta");

    tracing::info!("🚀 Servidor disponível em http://localhost:4001");

    // Inicia servidor
    axum::serve(listener, app).await.unwrap();
}