use crate::pg_db::PostgresDB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use crate::errors::ServerError;

// структура продукта
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub price: f64,
}

// структура продукта для изменения данных
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPayload {
    pub name: String,
    pub price: f64,
}

pub struct ProductModule {
    postgres_db: Arc<PostgresDB>,
}

impl ProductModule {
    // инициализация модуля модели продуктов
    pub fn new(postgres_db: Arc<PostgresDB>) -> Self {
        ProductModule { postgres_db }
    }

    // добавление пользователя
    pub async fn add_product(&self, product: Product) -> Result<(), ServerError> {
        let product_id = product.product_id;
        match self.postgres_db.insert_product(product).await {
            Ok(()) => {
                info!("Добавлен новый продукт с product_id: {}", product_id);
                Ok(())
            }
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }

    // изменение данных продукта
    pub async fn update_product(&self, product_id: i32, product_payload: ProductPayload) -> Result<(), ServerError> {
        match self.postgres_db.update_product(product_id, product_payload).await {
            Ok(()) => {
                info!("Обновлены данные продукта с product_id: {}",  product_id);
                Ok(())
            }
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }

    // удаление данных продукта
    pub async fn delete_product(&self, product_id: i32) -> Result<(), ServerError> {
        match self.postgres_db.delete_product(product_id).await {
            Ok(()) => {
                info!("Удалён продукт с product_id: {}", product_id);
                Ok(())
            }
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }
}
