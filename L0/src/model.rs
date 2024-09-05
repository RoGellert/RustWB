use serde::{Deserialize, Serialize};
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
    pub amount: u32,
    pub payment_dt: u32,
    pub bank: String,
    pub delivery_cost: u32,
    pub goods_total: u32,
    pub custom_fee: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub chrt_id: u32,
    pub track_number: String,
    pub price: u32,
    pub rid: String,
    pub name: String,
    pub sale: u32,
    pub size: String,
    pub total_price: u32,
    pub nm_id: u32,
    pub brand: String,
    pub status: u32,
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
    pub date_created: String,
    pub oof_shard: String,
}
