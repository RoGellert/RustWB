pub mod config;
pub mod db {
    pub mod postgres_db;
    // pub mod redis_db;
}
pub mod model;

use crate::config::DbConfig;
use crate::model::{OrdersModel};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let db_config = DbConfig::new();
    let orders_model: Arc<OrdersModel> = Arc::new(OrdersModel::new(&db_config).await.unwrap());

    orders_model
        .get_one_order_by_uuid(&Uuid::from_str("3f46be32-cc4d-408a-a31f-95a6ce17c035").unwrap())
        .await
        .unwrap();
    let orders = orders_model.get_all_orders().await.unwrap();
    println!("{:?}", &orders);
}
