//! инициализация и методы работы с базой данных Postgres

use crate::config::DbConfig;
use crate::data_types::{Post, PostPayload, User, UserLike, UserPayloadHashed};
use deadpool_postgres::{
    Config as DeadpoolConfig, CreatePoolError, GenericClient, ManagerConfig, Pool, RecyclingMethod,
    Runtime,
};
use serde_json::Value;
use std::error::Error;
use tokio_postgres::NoTls;
use uuid::Uuid;

// обёртка вокруг пула подключений
pub struct PostgresDB {
    pool: Pool,
}

// парсинг данных окружения и создания конфига для deadpool
fn create_deadpool_config(db_config: &DbConfig) -> DeadpoolConfig {
    let mut cfg = DeadpoolConfig::new();
    cfg.dbname = Some(db_config.pg_dbname.to_string());
    cfg.user = Some(db_config.pg_user.to_string());
    cfg.password = Some(db_config.pg_password.to_string());
    cfg.host = Some(db_config.pg_host.to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg
}

// методы инициализации и работы с базой данных Postgres
impl PostgresDB {
    // создание инстанса базы данных опираясь на конфиг
    pub async fn new(db_config: DbConfig) -> Result<Self, CreatePoolError> {
        // настройка конфига для подключения и пулинга
        let cfg = create_deadpool_config(&db_config);

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
            SELECT json_agg(result) FROM
            (SELECT * FROM users WHERE login = $1)
            result;
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
        let mut user_vec: Vec<User> = serde_json::from_value(user_json)?;
        let user = user_vec.remove(0);

        Ok(Some(user))
    }

    pub async fn insert_post(
        &self,
        user_uuid: Uuid,
        post_payload: PostPayload,
    ) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO posts
            (user_uuid,
            post_text,
            like_count)
        VALUES ($1, $2, $3);
        ";

        // выполнение запроса с нужными данными
        client
            .query(statement, &[&user_uuid, &post_payload.post_text, &0])
            .await?;

        Ok(())
    }

    pub async fn get_post_by_uuid(&self, post_uuid: Uuid) -> Result<Option<Post>, Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            SELECT json_agg(result) FROM
            (SELECT * FROM posts WHERE post_uuid = $1)
            result;
        ";

        // выполнение запроса с нужными данными
        let row = client.query_one(statement, &[&post_uuid]).await?;
        let post_json_option: Option<Value> = row.get(0);

        // если json пуст, возврат Ok(None)
        let post_json = match post_json_option {
            None => return Ok(None),
            Some(post_json) => post_json,
        };

        // десериализация
        let mut post_vec: Vec<Post> = serde_json::from_value(post_json)?;
        let post = post_vec.remove(0);

        Ok(Some(post))
    }

    pub async fn delete_post_by_uuid(&self, post_uuid: Uuid) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            DELETE FROM posts
            WHERE post_uuid = $1;
        ";

        // выполнение запроса с нужными данными
        client.query(statement, &[&post_uuid]).await?;

        Ok(())
    }

    // получение всех постов пользователя
    pub async fn get_all_posts_by_user_uuid(&self, user_uuid: Uuid) -> Result<Option<Vec<Post>>, Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            SELECT json_agg(result) FROM
            (SELECT * FROM posts WHERE user_uuid = $1)
            result;
        ";

        // выполнение запроса с нужными данными
        let row = client.query_one(statement, &[&user_uuid]).await?;
        let post_json_option: Option<Value> = row.get(0);

        // если json пуст, возврат Ok(None)
        let post_json = match post_json_option {
            None => return Ok(None),
            Some(post_json) => post_json,
        };

        // десериализация
        let posts_vec: Vec<Post> = serde_json::from_value(post_json)?;

        Ok(Some(posts_vec))
    }

    // получения лайка поста пользователем из таблицы user_likes
    pub async fn get_user_like(&self, user_uuid: Uuid, post_uuid: Uuid) -> Result<Option<UserLike>, Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            SELECT json_agg(result) FROM
            (SELECT * FROM user_likes WHERE
            user_uuid = $1 AND
            post_uuid = $2)
            result;
        ";

        // выполнение запроса с нужными данными
        let row = client.query_one(statement, &[&user_uuid, &post_uuid]).await?;
        let user_like_option: Option<Value> = row.get(0);

        // если json пуст, возврат Ok(None)
        let user_like_json = match user_like_option {
            None => return Ok(None),
            Some(user_like) => user_like,
        };

        // десериализация
        let mut user_like_vec: Vec<UserLike> = serde_json::from_value(user_like_json)?;
        let user_like = user_like_vec.remove(0);

        Ok(Some(user_like))
    }

    pub async fn increment_likes_by_uuid(&self, post_uuid: Uuid) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            UPDATE posts
            SET like_count = like_count + 1
            WHERE post_uuid = $1;
        ";

        // выполнение запроса с нужными данными
        client.query(statement, &[&post_uuid]).await?;

        Ok(())
    }

    pub async fn insert_user_to_like(&self, user_uuid: Uuid, post_uuid: Uuid) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO user_likes
            (user_uuid,
            post_uuid)
            VALUES ($1, $2);
        ";

        // выполнение запроса с нужными данными
        client.query(statement, &[&user_uuid, &post_uuid]).await?;

        Ok(())
    }
}
