use axum::Json;
use futures_util::StreamExt;
use lapin::{options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions}, types::FieldTable};
use tokio::task;
use tracing::info;

use crate::{repository::EventRepository, services::{Connect, Database, EventQueue}};

const QUEUE_NAME: &str = "stock-manager-events";
const CONSUMER_TAG: &str = "event-processor";

pub fn start() {
    task::spawn(async move {
        let pool = Database::connect().await;
        let queue = EventQueue::connect().await;

        queue.queue_declare(QUEUE_NAME, QueueDeclareOptions::default(), FieldTable::default()).await.unwrap();

        let mut consumer = queue.basic_consume(
            QUEUE_NAME,
            CONSUMER_TAG,
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await.expect("Erro ao iniciar consumidor de mensagens");

        info!("Consumer started for queue {}", QUEUE_NAME);

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");

            let Json(data) = Json::from_bytes(&delivery.data.to_owned()).expect("Erro ao deserializar mensagem");

            info!("{:?}", data);

            EventRepository::create(&pool, data).await.unwrap();

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("ack");
        }
    });
}