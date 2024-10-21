use kafka::producer::{Producer, Record};
use std::time::Duration;
use crate::config::KafkaConfig;

pub struct KafkaProducer {
    producer: Producer,
}

impl KafkaProducer {
    pub fn new(kafka_config: &KafkaConfig) -> Self {
        let config_string = format!("{}:{}", kafka_config.kafka_host, kafka_config.kafka_port);

        let producer = Producer::from_hosts(vec!(config_string))
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(kafka::client::RequiredAcks::One)
            .create()
            .expect("не удалось подключится к сервису Kafka");

        KafkaProducer { producer }
    }

    pub fn send_message(&mut self, topic: &str, key: &str, message: &str) {
        let _ = self.producer.send(&Record {
            topic,
            partition: -1,
            key: key.as_bytes(),
            value: message.as_bytes(),
        });
    }
}