use futures::channel::mpsc;
use futures::{stream, StreamExt, TryStreamExt, FutureExt};
use tokio_postgres::{AsyncMessage, NoTls};
use crate::config::Config;
use crate::kafka_module::KafkaModule;
use tracing::{info, Level};

mod config;
mod kafka_module;

#[tokio::main]
async fn main() {
    // создание конфига и чтение переменных окружения
    let config = Config::new();

    // подключение к сервису Kafka и создание модуля
    let kafka_module = KafkaModule::from_config(&config.kafka_config);

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let topic_name = "server_notifications".to_string();
    // создание новой темы сообщений в сервисе kafka
    kafka_module.create_topic(&topic_name).await.expect("не удалось создать новую тему сообщений в сервисе Kafka");

    // создание конфига базы
    let (client, mut connection) = tokio_postgres::connect(
        &format!(
            "host={} user={} password={} dbname={}",
            config.db_config.pg_host, config.db_config.pg_user, config.db_config.pg_password, config.db_config.pg_dbname
        ),
        NoTls,
    )
        .await
        .expect("не удалось создать конфиг поключения к базе данных");

    // создание канала, в который будут приходить оповещения об изменениях в базе
    let (tx, mut rx) = mpsc::unbounded();
    let stream =
        stream::poll_fn(move |cx| connection.poll_message(cx)).map_err(|e| panic!("{}", e));
    let connection = stream.forward(tx).map(|r| r.unwrap());
    // подключение к базе
    tokio::spawn(connection);

    // запрос к базе данных на прослушивание изменений
    let query = "LISTEN user_changes; LISTEN product_changes;";
    client.batch_execute(query).await.expect("не удалось запустить прослушивание и обработку запросов");
    info!("прослушивание и обработка запросов запущены");

    // прослушивание сообщений
    loop  {
        match rx.try_next() {
            Ok(Some(AsyncMessage::Notification(notification))) => {
                info!("получено сообщение об изменении, содержимое: {}", notification.payload());
                kafka_module.send_message_to_topic(&topic_name, notification.channel().to_string(), notification.payload().to_string()).await.expect("не удалось отправить сообщение")
            },
            Ok(None) => {
                break
            }
            _ => {}
        }
    }

    drop(client);

    info!("прослушивание и обработка запросов остановлены");
}
