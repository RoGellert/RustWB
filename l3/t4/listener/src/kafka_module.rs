use crate::config::KafkaConfig;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use std::time::Duration;
use tracing::{error, info};

pub struct KafkaModule {
    admin_client: AdminClient<DefaultClientContext>,
    producer: FutureProducer,
}

impl KafkaModule {
    // подключение к сервису опираясь на конфиг
    pub fn from_config(kafka_config: &KafkaConfig) -> Self {
        // создание адреса брокера
        let broker = format!("{}:{}", kafka_config.kafka_host, kafka_config.kafka_port);

        // создание клиента админа
        let admin_client: AdminClient<_> = ClientConfig::new()
            .set("bootstrap.servers", &broker)
            .create()
            .expect("не удалось создать клиента админа");

        // создание Kafka producer
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &broker)
            .create()
            .expect("не удалось создать Producer сервиса Kafka");

        KafkaModule {
            admin_client,
            producer,
        }
    }

    // создание темы
    pub async fn create_topic(&self, topic_name: &str) -> Result<(), KafkaError> {
        // новая тема сервиса Kafka
        let new_topic = NewTopic::new(topic_name, 1, TopicReplication::Fixed(1));
        let res = self
            .admin_client
            .create_topics(
                &[new_topic],
                &AdminOptions::new().operation_timeout(Some(Timeout::from(Duration::from_secs(5)))),
            )
            .await;

        // обработка результата
        match res {
            Ok(_) => {
                info!("Тема под названием {} создана", topic_name);
                Ok(())
            }
            Err(err) => {
                error!(
                    "Не удалось создать Topic под названием {}, ошибка: {:?}",
                    topic_name, &err
                );
                Err(err)
            }
        }
    }

    // отправка сообщения в тему
    pub async fn send_message_to_topic(
        &self,
        topic_name: &str,
        key: String,
        value: String,
    ) -> Result<(), KafkaError> {
        // отправка сообщения
        let res = self
            .producer
            .send(
                FutureRecord::to(topic_name).payload(&value).key(&key),
                Duration::from_secs(0),
            )
            .await;

        // обработка результата
        match res {
            Ok(_) => {
                info!(
                    "Сообщение c ключем: ' {} ' и содержанием ' {} ' отправлено в сервис kafka",
                    key, value
                );
                Ok(())
            }
            Err((err, _)) => {
                error!(
                    "Не удалось отправить сообщение с ключем ' {} ', ошибка: {}",
                    key, &err
                );
                Err(err)
            }
        }
    }
}
