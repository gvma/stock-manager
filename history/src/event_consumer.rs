use axum::Json;
use futures_util::StreamExt;
use lapin::{options::{BasicAckOptions, BasicConsumeOptions}, types::FieldTable};
use tokio::task;
use tracing::info;

use crate::{repository::EventRepository, services::{Connect, Database, EventQueue}};

pub fn start() {
    task::spawn(async move {
        info!("will consume");
        let pool = Database::connect().await;
        let queue = EventQueue::connect().await;

        let mut consumer = queue.basic_consume(
            "stock-manager-auth",
            "auth-event-processor",
            BasicConsumeOptions::default(),
            FieldTable::default()
        ).await.expect("Erro ao iniciar consumidor de mensagens do auth");

        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");

            let Json(data) = Json::from_bytes(&delivery.data.to_owned()).expect("Erro ao deserializar mensagem do auth");

            println!("{:?}", data);

            EventRepository::create(&pool, data).await.unwrap();

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("ack");
        }
    });
}