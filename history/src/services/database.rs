use std::env;

use mongodb::Client;

use super::Connect;

pub struct Database;

impl Connect<mongodb::Database> for Database {
    async fn connect() -> mongodb::Database {
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI não definido");
        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME não definido");

        let client = Client::with_uri_str(uri).await.expect("Failed to connect to MongoDB");

        client.database(&database_name)
    }
}