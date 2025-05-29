use std::env;

use lapin::{Channel, Connection, ConnectionProperties};

use super::Connect;

pub struct EventQueue;

impl Connect<Channel> for EventQueue {
    async fn connect() -> Channel {
        let rabbitmq_url = env::var("RABBITMQ_URL").expect("RABBITMQ_URL não definido");    

        let connection_options = ConnectionProperties::default();

        Connection::connect(&rabbitmq_url, connection_options)
            .await
            .expect("Erro ao conectar com o sistema de mensagens")
            .create_channel()
            .await
            .expect("Erro ao criar canal de mensagens")
    }
}