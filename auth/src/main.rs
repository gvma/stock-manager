mod db;
mod routes;
mod repository;
mod domain;
mod handlers;

use routes::routes;
use tracing_subscriber;
use axum::{Extension};
use db::{start_db_pool, start_event_queue};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // Inicializa os logs
    tracing_subscriber::fmt::init();
    
    dotenv().ok();

    let pool = start_db_pool().await;
    let queue = start_event_queue().await;

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