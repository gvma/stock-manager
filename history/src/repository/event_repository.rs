use crate::domain::Event;
use mongodb::{bson::doc, error::Error, Cursor, Database};

pub struct EventRepository;

impl EventRepository {
    pub async fn list(pool: &Database) -> Result<Cursor<Event>, Error> {
        let events = pool.collection("events").find(doc! {}).await;

        events
    }
}
