use chrono::{serde::ts_seconds, DateTime, Utc};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub previous_version: Option<Document>,
    pub current_version: Document,
    pub actor: String,

    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub occurred_at: DateTime<Utc>
}