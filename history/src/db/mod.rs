use std::env;

use mongodb::{Client, Database};

pub async fn start_db_pool() -> Database {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI não definido");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME não definido");

    let client = Client::with_uri_str(uri).await.expect("Failed to connect to MongoDB");

    client.database(&database_name)
}