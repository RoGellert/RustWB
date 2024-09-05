use crate::config::DbConfig;
use log::error;
use std::sync::Arc;
use tokio_postgres::{Client, Error as PostgresError, NoTls};

pub struct PostgresDB {
    client: Arc<Client>,
}

impl PostgresDB {
    pub async fn new(db_config: &DbConfig) -> Result<Self, PostgresError> {
        let (client, connection) = tokio_postgres::connect(
            &format!(
                "host={} user={} password={} dbname={}",
                db_config.pg_host, db_config.pg_user, db_config.pg_password, db_config.pg_dbname
            ),
            NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("connection error: {e}");
            }
        });

        Ok(PostgresDB {
            client: Arc::new(client),
        })
    }
    pub async fn create_table(&self) -> Result<(), PostgresError> {
        self.client
            .query(r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#, &[])
            .await?;

        self.client
            .query(
                "CREATE TABLE orders (
                            order_uid UUID PRIMARY KEY,
                            track_number VARCHAR,
                            entry VARCHAR,
                            payment VARCHAR,
                            locale VARCHAR,
                            internal_signature VARCHAR,
                            customer_id VARCHAR,
                            delivery_service VARCHAR,
                            shardkey VARCHAR,
                            sm_id integer,
                            date_created VARCHAR,
                            oof_shard VARCHAR
                        );",
                &[],
            )
            .await?;

        Ok(())
    }

    pub async fn drop_table(&self, table_name: &str) -> Result<(), PostgresError> {
        self.client
            .query(&format!("DROP TABLE {};", table_name), &[])
            .await?;

        Ok(())
    }
}
