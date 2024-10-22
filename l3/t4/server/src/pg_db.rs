use crate::config::DbConfig;
use crate::modules::product_module::{Product, ProductPayload};
use crate::modules::user_module::{User, UserPayload};
use deadpool_postgres::{
    Config as DeadpoolConfig, CreatePoolError, ManagerConfig, Pool, RecyclingMethod, Runtime,
};
use std::error::Error;
use tokio_postgres::NoTls;

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

    // добавление пользователя в базу
    pub async fn insert_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO users
            (user_id,
            name,
            email)
        VALUES ($1, $2, $3);
        ";

        // выполнение запроса с нужными данными
        client
            .query(statement, &[&user.user_id, &user.name, &user.email])
            .await?;

        Ok(())
    }

    // изменение данных пользователя в базе
    pub async fn update_user(&self, user_id: i32, user_payload: UserPayload) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            UPDATE users
            SET name = $1, email = $2
            WHERE user_id = $3
        ";

        // выполнение запроса с нужными данными
        client
            .query(statement, &[&user_payload.name, &user_payload.email, &user_id])
            .await?;

        Ok(())
    }

    // удаление данных о пользователе из базы
    pub async fn delete_user(
        &self,
        user_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            DELETE FROM users
            WHERE user_id = $1
        ";

        // выполнение запроса с нужными данными
        client
            .query(
                statement,
                &[
                    &user_id,
                ],
            )
            .await?;

        Ok(())
    }

    // добавление продукта в базу
    pub async fn insert_product(&self, product: Product) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO products
            (product_id,
            name,
            price)
        VALUES ($1, $2, $3);
        ";

        // выполнение запроса с нужными данными
        client
            .query(statement, &[&product.product_id, &product.name, &product.price])
            .await?;

        Ok(())
    }

    // изменение данных продукта в базе
    pub async fn update_product(&self, product_id: i32, product_payload: ProductPayload) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            UPDATE products
            SET name = $1, price = $2
            WHERE product_id = $3
        ";

        // выполнение запроса с нужными данными
        client
            .query(statement, &[&product_payload.name, &product_payload.price, &product_id])
            .await?;

        Ok(())
    }

    // удаление данных о продукте из базы
    pub async fn delete_product(
        &self,
        product_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            DELETE FROM products
            WHERE product_id = $1
        ";

        // выполнение запроса с нужными данными
        client
            .query(
                statement,
                &[
                    &product_id,
                ],
            )
            .await?;

        Ok(())
    }
}
