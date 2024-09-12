//! инициализация и методы работы с базой данных redis для кэширования
use crate::config::DbConfig;
use crate::model::Order;
use deadpool_redis::{redis::cmd, Config, CreatePoolError, Pool, Runtime};
use std::error::Error;

// обёртка вокруг пула подключений
pub struct RedisDB {
    pool: Pool,
}

// методы работы и инициализыции базы данных redis
impl RedisDB {
    // инициализация подключения к базе данных redis
    pub async fn new(db_config: &DbConfig) -> Result<RedisDB, CreatePoolError> {
        // конфигурация на основе переменных окружения
        let config = Config::from_url(format!(
            "redis://{}:{}",
            &db_config.redis_host, &db_config.redis_port
        ));

        // создания пула подключений
        let pool = config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self { pool })
    }

    // добавление в кэш по ключу
    pub async fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // добавление в кэш по ключу
        cmd("SET")
            .arg(&[key, value])
            .query_async::<()>(&mut conn)
            .await?;

        // время жизни данных в кэшэ
        let ttl = "100";
        cmd("EXPIRE")
            .arg(&[key, ttl])
            .query_async::<()>(&mut conn)
            .await?;

        Ok(())
    }

    // удаление из кэша по ключу
    pub async fn del(&self, key: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // удаление из кэша по ключу
        cmd("DEL").arg(&[key]).query_async(&mut conn).await?;

        Ok(())
    }

    // получение одного заказа по ключу
    pub async fn get_order(
        &self,
        key: &str,
    ) -> Result<Option<Order>, Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // запрос к Redis на получение одного заказа
        let redis_result: Option<String> = cmd("GET").arg(&[key]).query_async(&mut conn).await?;

        // возращение None если ключа нет в базе
        let data = match redis_result {
            Some(res) => res,
            None => return Ok(None),
        };

        // десериализация одного заказа
        let order: Order = serde_json::from_str(&data)?;
        // let order: Order = orders.remove(0);

        Ok(Some(order))
    }

    // получение всех заказов
    pub async fn get_all_orders(&self) -> Result<Option<Vec<Order>>, Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let mut conn = self.pool.get().await?;

        // запрос к Redis на получение всех заказов
        let redis_result: Option<String> =
            cmd("GET").arg(&["orders"]).query_async(&mut conn).await?;

        // возращение None если ключа нет в базе
        let data = match redis_result {
            Some(res) => res,
            None => return Ok(None),
        };

        // десериализация всех заказов
        let orders: Vec<Order> = serde_json::from_str(&data)?;
        Ok(Some(orders))
    }
}
