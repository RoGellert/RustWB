//! инициализация и методы работы с базой данных redis для кэширования
use crate::config::RedisConfig;
use deadpool_redis::redis::{AsyncCommands};
use deadpool_redis::{ Config, CreatePoolError, Pool, Runtime};
use std::error::Error;
use uuid::Uuid;
use crate::modules::event_manager::Event;

// обёртка вокруг пула подключений
pub struct RedisDB {
    pool: Pool,
}

// методы работы и инициализыции базы данных redis
impl RedisDB {
    // инициализация подключения к базе данных redis
    pub async fn new(db_config: &RedisConfig) -> Result<RedisDB, CreatePoolError> {
        // конфигурация на основе переменных окружения
        let config = Config::from_url(format!(
            "redis://{}:{}",
            &db_config.redis_host, &db_config.redis_port
        ));

        // создания пула подключений
        let pool = config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self { pool })
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
        conn.rpush(format!("{}:subscriptions", &user_uuid), event_type).await?;

        Ok(())
    }

    // добавление нового ивента
    pub async fn add_event(
        &self,
        event: Event,
        event_serialised: String
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // добавление подписок в лист подписок
        conn.rpush(format!("events:{}", &event.event_type), event_serialised).await?;

        Ok(())
    }

    // получение всех ивентов по категории
    pub async fn get_events_by_category(
        &self,
        category: String
    ) -> Result<Option<Vec<String>>, Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // получение всех ивентов по категории
        let events: Vec<String> =
            conn.lrange(format!("{}:subscriptions", &category), 0, -1).await?;

        if events.is_empty() {
            return Ok(None)
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
        let subscriptions: Vec<String> =
            conn.lrange(format!("{}:subscriptions", &user_uuid), 0, -1).await?;

        if subscriptions.is_empty() {
            return Ok(None)
        }

        Ok(Some(subscriptions))
    }
}
