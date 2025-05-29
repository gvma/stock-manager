use chrono::{serde::ts_seconds, DateTime, Utc};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub previous_version: Option<Document>,
    pub current_version: Document,
    pub actor: String,

    #[serde(with = "ts_seconds")]
    pub occurred_at: DateTime<Utc>
}