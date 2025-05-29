mod services;
mod domain;
mod handlers;
mod repository;
mod routes;

use axum::Extension;
use services::{Connect, Database};
use dotenvy::dotenv;
use routes::routes;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inicializa os logs
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let pool = Database::connect().await;

    // Constrói as rotas do app com acesso ao DB nas rotas (extension e layer)
    let app = routes().layer(Extension(pool));

    // Abre o socket
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("Erro ao bindar porta");

    tracing::info!("🚀 Servidor disponível em http://localhost:4000");

    // Inicia servidor
    axum::serve(listener, app).await.unwrap();
}
