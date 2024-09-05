use crate::config::DbConfig;
use crate::model::{Delivery, Order};
use deadpool_postgres::{
    Config as DeadpoolConfig, CreatePoolError, ManagerConfig, Pool, RecyclingMethod, Runtime, PoolConfig
};
use tokio_postgres::NoTls;
use uuid::Uuid;

pub struct PostgresDB {
    pool: Pool,
}

fn create_deadpool_config(db_config: &DbConfig) -> DeadpoolConfig {
    let mut cfg = DeadpoolConfig::new();
    cfg.dbname = Some((db_config.pg_dbname).to_string());
    cfg.user = Some((db_config.pg_user).to_string());
    cfg.password = Some((db_config.pg_password).to_string());
    cfg.dbname = Some((db_config.pg_dbname).to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg
}

impl PostgresDB {
    pub async fn new(db_config: &DbConfig) -> Result<Self, CreatePoolError> {
        // настройка конфига для подключения и пулинга
        let cfg = create_deadpool_config(db_config);

        // создание пула подключений
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

        Ok(Self { pool })
    }

    pub async fn insert_order(&self, order: &Order) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO orders
            (order_uid,
            track_number,
            entry,
            locale,
            internal_signature,
            customer_id,
            delivery_service,
            shardkey,
            sm_id,
            date_created,
            oof_shard)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        ";

        client
            .query(
                statement,
                &[
                    &order.order_uid,
                    &order.track_number,
                    &order.entry,
                    &order.locale,
                    &order.internal_signature,
                    &order.customer_id,
                    &order.delivery_service,
                    &order.shardkey,
                    &order.sm_id,
                    &order.date_created,
                    &order.oof_shard,
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn insert_delivery(
        &self,
        delivery: &Delivery,
        order_uid: &Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO deliveries
            (name,
            phone,
            zip,
            city,
            address,
            region,
            email,
            order_uid)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
        ";

        client
            .query(
                statement,
                &[
                    &delivery.name,
                    &delivery.phone,
                    &delivery.zip,
                    &delivery.city,
                    &delivery.address,
                    &delivery.region,
                    &delivery.email,
                    order_uid,
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn get_all_orders(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;

        let statement = "
            SELECT * FROM orders;
        ";

        let rows = client.query_one(statement, &[]).await?;

        let order_uid: Uuid = rows.get("order_uid");

        println!("{:?}", &order_uid);

        Ok(())
    }
}
