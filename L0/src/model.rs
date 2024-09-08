use crate::config::DbConfig;
use crate::db::postgres_db::PostgresDB;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Delivery {
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub city: String,
    pub address: String,
    pub region: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payment {
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: i32,
    pub payment_dt: i32,
    pub bank: String,
    pub delivery_cost: i32,
    pub goods_total: i32,
    pub custom_fee: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub chrt_id: i32,
    pub track_number: String,
    pub price: i32,
    pub rid: String,
    pub name: String,
    pub sale: i32,
    pub size: String,
    pub total_price: i32,
    pub nm_id: i32,
    pub brand: String,
    pub status: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_uid: Uuid,
    pub track_number: String,
    pub entry: String,
    pub delivery: Delivery,
    pub payment: Payment,
    pub items: Vec<Item>,
    pub locale: String,
    pub internal_signature: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: i32,
    pub date_created: NaiveDateTime,
    pub oof_shard: String,
}

pub struct OrdersModel {
    postgres_instance: PostgresDB,
}

impl OrdersModel {
    pub async fn new(db_config: &DbConfig) -> Result<Self, Box<dyn Error>> {
        // инициализация базы данных
        let postgres_instance = PostgresDB::new(&db_config).await?;

        Ok(OrdersModel { postgres_instance })
    }

    pub async fn insert_order(&self, order: &Order) -> Result<(), Box<dyn Error>> {
        self.postgres_instance.insert_order(order).await?;
        Ok(())
    }

    pub async fn get_all_orders(&self) -> Result<Vec<Order>, Box<dyn Error>> {
        let orders: Vec<Order> = self.postgres_instance.get_all_orders().await?;
        Ok(orders)
    }

    pub async fn get_one_order_by_uuid(&self, order_uuid: &Uuid) -> Result<Order, Box<dyn Error>> {
        let order: Order = self
            .postgres_instance
            .get_one_order_by_uuid(order_uuid)
            .await?;
        Ok(order)
    }
}
