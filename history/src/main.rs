mod services;
mod domain;
mod handlers;
mod repository;
mod routes;
mod event_consumer;

use axum::Extension;
use services::{Connect, Database, EventQueue};
use dotenvy::dotenv;
use routes::routes;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inicializa os logs
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let pool = Database::connect().await;
    let queue = EventQueue::connect().await;

    // Constrói as rotas do app com acesso ao DB nas rotas (extension e layer)
    let app = routes().layer(Extension(pool)).layer(Extension(queue));

    // Abre o socket
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4002")
        .await
        .expect("Erro ao bindar porta");

    tracing::info!("🚀 Servidor disponível em http://localhost:4002");

    event_consumer::start();

    // Inicia servidor
    axum::serve(listener, app).await.unwrap();
}
