mod db;
mod domain;
mod handlers;
mod repository;
mod routes;

use axum::Extension;
use db::start_db_pool;
use dotenvy::dotenv;
use routes::routes;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inicializa os logs
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let pool = start_db_pool().await;

    // Constrói as rotas do app com acesso ao DB nas rotas (extension e layer)
    let app = routes().layer(Extension(pool));

    // Abre o socket
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4002")
        .await
        .expect("Erro ao bindar porta");

    tracing::info!("🚀 Servidor disponível em http://localhost:4002");

    // Inicia servidor
    axum::serve(listener, app).await.unwrap();
}
