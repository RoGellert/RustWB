use crate::pg_db::PostgresDB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPayload {
    pub name: String,
    pub price: f32,
}

pub struct ProductModule {
    postgres_db: Arc<PostgresDB>,
}

impl ProductModule {
    // инициализация модуля модели продуктов
    pub fn new(postgres_db: Arc<PostgresDB>) -> Self {
        ProductModule { postgres_db }
    }
}
