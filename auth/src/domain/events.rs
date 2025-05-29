use chrono::{serde::ts_seconds, DateTime, Utc};
use lapin::{options::BasicPublishOptions, publisher_confirm::PublisherConfirm, BasicProperties, Channel, Error};
use serde::Serialize;

use super::User;

const QUEUE_NAME: &str = "stock-manager-events";

pub trait PublishableEvent<T> {
    fn new(previous_version: Option<T>, current_version: Option<T>, actor: String) -> Self;
    fn event_name() -> String;
    fn event_time() -> DateTime<Utc> { Utc::now() }
    async fn publish(&self, queue: &Channel) -> Result<PublisherConfirm, Error> where Self: Serialize {
        queue.basic_publish(
            "",
            QUEUE_NAME,
            BasicPublishOptions::default(),
            &serde_json::to_vec(self).unwrap(),
            BasicProperties::default()
        ).await
    }
}

#[derive(Serialize)]
pub struct UserCreated {
    name: String,
    #[serde(with = "ts_seconds")]
    occurred_at: DateTime<Utc>,

    pub previous_version: Option<User>,
    pub current_version: Option<User>,
    pub actor: String
}

impl PublishableEvent<User> for UserCreated {
    fn new(previous_version: Option<User>, current_version: Option<User>, actor: String) -> Self {
        Self {
            name: Self::event_name(),
            occurred_at: Self::event_time(),
            previous_version,
            current_version,
            actor
        }
    }

    fn event_name() -> String {
        "auth:user:created".to_owned()
    }
}