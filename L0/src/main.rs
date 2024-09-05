mod config;
pub mod db {
    pub mod postgres_db;
    pub mod redis_db;
}
mod model;

use crate::config::DbConfig;
use crate::db::postgres_db::PostgresDB;

#[tokio::main]
async fn main() {
    let db_config = DbConfig::new();

    let postgres_instance: PostgresDB = PostgresDB::new(&db_config).await.unwrap();

    postgres_instance.create_table().await.unwrap();
}
