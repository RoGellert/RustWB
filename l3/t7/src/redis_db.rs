//! инициализация и методы работы с базой данных redis для кэширования
use crate::config::RedisConfig;
use crate::errors::ServerError;
use crate::modules::event_manager::Event;
use axum::extract::ws::{Message, WebSocket};
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Config, CreatePoolError, Pool, Runtime};
use futures_util::StreamExt;
use std::error::Error;
use tokio::sync::broadcast::Receiver;
use tracing::info;
use uuid::Uuid;

// обёртка вокруг пула подключений
pub struct RedisDB {
    // пул подлючений
    pool: Pool,
    // строка конфига для открытия индивидуальных подключений
    config_string: String,
}

// методы работы и инициализыции базы данных redis
impl RedisDB {
    // инициализация подключения к базе данных redis
    pub async fn new(db_config: &RedisConfig) -> Result<RedisDB, CreatePoolError> {
        let config_string = format!(
            "redis://{}:{}",
            &db_config.redis_host, &db_config.redis_port
        );

        // конфигурация на основе переменных окружения
        let config = Config::from_url(config_string.clone());

        // создания пула подключений
        let pool = config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self {
            pool,
            config_string,
        })
    }

    // добавление новой подписки
    pub async fn subscribe(
        &self,
        user_uuid: Uuid,
        event_type: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // добавление подписок в лист подписок
        conn.rpush(format!("{}:subscriptions", &user_uuid), event_type)
            .await?;

        Ok(())
    }

    // добавление нового события
    pub async fn add_event(
        &self,
        event: Event,
        event_serialised: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // добавление подписок в лист подписок
        conn.rpush(format!("events:{}", &event.event_type), event_serialised)
            .await?;

        Ok(())
    }

    // получение всех событий по типу
    pub async fn get_events_by_category(
        &self,
        category: String,
    ) -> Result<Option<Vec<String>>, Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // получение всех ивентов по категории
        let events: Vec<String> = conn.lrange(format!("events:{}", &category), 0, -1).await?;

        // проверка пуст ли возвращаемый вектор
        if events.is_empty() {
            return Ok(None);
        }

        Ok(Some(events))
    }

    // получение всех подписок
    pub async fn get_all_subscriptions_by_user_uuid(
        &self,
        user_uuid: Uuid,
    ) -> Result<Option<Vec<String>>, Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // получение всех подписок
        let subscriptions: Vec<String> = conn
            .lrange(format!("{}:subscriptions", &user_uuid), 0, -1)
            .await?;

        // проверка есть ли у пользователя подписки
        if subscriptions.is_empty() {
            return Ok(None);
        }

        Ok(Some(subscriptions))
    }

    // нотификация через WebSocket
    pub async fn handle_pub_sub(
        &self,
        mut socket: WebSocket,
        user_subscriptions: Vec<String>,
        mut subscription_change_receiver: Receiver<String>,
        user_uuid: Uuid,
    ) -> Result<(), ServerError> {
        // создание потока обрабатывающего PubSub из Redis
        let client = redis::Client::open(self.config_string.clone())
            .map_err(|err| ServerError::Redis(Box::new(err)))?;
        let (mut sink, mut stream) = client
            .get_async_pubsub()
            .await
            .map_err(|err| ServerError::Redis(Box::new(err)))?
            .split();

        // подписка на каналы пользователя из базы redis
        for subscription in user_subscriptions {
            sink.subscribe(&subscription)
                .await
                .map_err(|err| ServerError::Redis(Box::new(err)))?;
        }

        info!("открыт WebSocket пользователя {}", user_uuid);
        loop {
            tokio::select! {
                // если получен новый канал для подписки - подписка на Redis PubSub
                new_channel_res = subscription_change_receiver.recv() => {
                    match new_channel_res {
                        Ok(new_channel) => {
                            if let Err(err) = sink.subscribe(&new_channel).await {
                                return Err(ServerError::Redis(Box::new(err)))
                            };

                            info!("сервер получил уведомление о подписке пользователя {} на уведомления типа {}", user_uuid, new_channel)
                        },
                        Err(_) => {
                            return Err(ServerError::Broadcast(format!("канал уведомлений пользователя {} закрыт", user_uuid)))
                        }
                    }
                }

                // если получено сообщение из Redis PubSub - отправка в WebSocket
                Some(msg) = stream.next() => {
                    let payload = match msg.get_payload() {
                        Err(err) => return Err(ServerError::Redis(Box::new(err))),
                        Ok(payload) => payload
                    };

                    info!("сервером получено и обработано уведомление о новом событии: {}", &payload);

                    if socket.send(Message::Text(payload)).await.is_err() {
                        return Err(ServerError::WebSocket(format!("не удалось отправить сообщение в WebSocket пользователю {}", user_uuid)))
                    }
                }

                // если сокет закрыт - остановка обработки
                result = socket.recv() => {
                    if result.is_none() {
                        break;
                    }
                }
            }
        }

        info!("закрыт WebSocket пользователя {}", user_uuid);
        Ok(())
    }

    pub async fn publish_message(
        &self,
        channel_name: String,
        message_content: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // публикация сообщения
        conn.publish(channel_name, message_content).await?;

        Ok(())
    }
}
