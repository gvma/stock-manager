use crate::domain::Event;
use mongodb::{bson::doc, error::Error, Cursor, Database};

pub struct EventRepository;

impl EventRepository {
    pub async fn list(pool: &Database) -> Result<Cursor<Event>, Error> {
        let events = pool.collection("events").find(doc! {}).await;

        events
    }

    pub async fn create(pool: &Database, event: Event) -> Result<Option<Event>, Error> {
        match pool.collection("events").insert_one(event).await {
            Ok(data) => pool.collection("events").find_one(doc! { "_id": data.inserted_id }).await,
            Err(err) => Err(err)
        }
    }
}
