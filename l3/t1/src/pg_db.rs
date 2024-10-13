//! инициализация и методы работы с базой данных Postgres

use crate::config::DbConfig;
use crate::data_types::{User, UserPayloadHashed};
use deadpool_postgres::{
    Config as DeadpoolConfig, CreatePoolError, GenericClient, ManagerConfig, Pool, RecyclingMethod,
    Runtime,
};
use serde_json::Value;
use std::error::Error;
use tokio_postgres::NoTls;

// обёртка вокруг пула подключений
pub struct PostgresDB {
    pool: Pool,
}

// парсинг данных окружения и создания конфига для deadpool
fn create_deadpool_config(db_config: &DbConfig) -> DeadpoolConfig {
    let mut cfg = DeadpoolConfig::new();
    cfg.dbname = Some((db_config.pg_dbname).to_string());
    cfg.user = Some((db_config.pg_user).to_string());
    cfg.password = Some((db_config.pg_password).to_string());
    cfg.host = Some((db_config.pg_host).to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg
}

// методы инициализации и работы с базой данных Postgres
impl PostgresDB {
    // создание инстанса базы данных опираясь на конфиг
    pub async fn new(db_config: &DbConfig) -> Result<Self, CreatePoolError> {
        // настройка конфига для подключения и пулинга
        let cfg = create_deadpool_config(db_config);

        // создание пула подключений
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

        Ok(Self { pool })
    }

    pub async fn insert_user(
        &self,
        user_payload_hashed: UserPayloadHashed,
    ) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO users
            (login,
            password_hash)
        VALUES ($1, $2);
        ";

        // выполнение запроса с нужными данными
        client
            .query(
                statement,
                &[
                    &user_payload_hashed.login,
                    &user_payload_hashed.password_hash,
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn get_user_by_login(&self, login: &str) -> Result<Option<User>, Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            SELECT * FROM users
            WHERE login = &1
        ";

        // выполнение запроса с нужными данными
        let row = client.query_one(statement, &[&login]).await?;

        let user_json_option: Option<Value> = row.get(0);

        // если json пуст, возврат Ok(None)
        let user_json = match user_json_option {
            None => return Ok(None),
            Some(user_json) => user_json,
        };

        // десериализация
        let user: User = serde_json::from_value(user_json)?;

        Ok(Some(user))
    }
}
