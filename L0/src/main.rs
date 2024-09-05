mod config;
pub mod db {
    pub mod postgres_db;
    // pub mod redis_db;
}
mod model;

use crate::config::DbConfig;
use crate::db::postgres_db::PostgresDB;
use crate::model::Order;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    let db_config = DbConfig::new();

    let postgres_instance: PostgresDB = PostgresDB::new(&db_config).await.unwrap();

    let mut file = File::open("json_files/model.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let order: Order = serde_json::from_str(&contents).unwrap();

    postgres_instance.insert_order(&order).await.unwrap();
    postgres_instance.insert_delivery(&order.delivery, &order.order_uid).await.unwrap();
    postgres_instance.get_all_orders().await.unwrap();
}
