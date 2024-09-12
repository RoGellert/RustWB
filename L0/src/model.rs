//! декларация модели данных, возможных ошибок сервера и основной логики модели заказов
use crate::config::DbConfig;
use crate::db::postgres_db::PostgresDB;
use crate::db::redis_db::RedisDB;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tokio::time::timeout;
use tracing::{error, info, warn};
use uuid::Uuid;

// структура доставки
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Delivery {
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub city: String,
    pub address: String,
    pub region: String,
    pub email: String,
}

// структура оплаты
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
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

// структура вещи
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
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

// структура заказа
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
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

// потенциальные ошибки
pub enum ServerError {
    NotFound(String),
    PostgresError(Box<dyn Error>),
    RedisError(Box<dyn Error>),
    TimeoutError(String),
    SerializationError(String),
    UnknownError,
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::NotFound(text) => {
                warn!("Данные по запросу не найдены: {:?}", text);
                (StatusCode::NOT_FOUND, format!("Данные по запросу не найдены: {:?}", text)).into_response()
            }
            ServerError::PostgresError(err) => {
                error!("Ошибка базы данных Postgres {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка базы данных Postgres {:?}", err),
                )
                    .into_response()
            }
            ServerError::RedisError(err) => {
                error!("Ошибка базы данных Redis {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка базы данных Redis {:?}", err),
                )
                    .into_response()
            }
            ServerError::TimeoutError(text) => {
                error!("Тайм-аут запроса {:?}", text);
                (
                    StatusCode::REQUEST_TIMEOUT,
                    format!("Тайм-аут запроса: {:?}", text),
                )
                    .into_response()
            }
            ServerError::SerializationError(text) => {
                error!("Ошибка сериализации в запросе {:?}", text);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Ошибка десериализации в запросе{:?}", text)).into_response()
            }
            ServerError::UnknownError => {
                error!("Неизвестная ошибка");
                (StatusCode::INTERNAL_SERVER_ERROR, "Неизвестная ошибка").into_response()
            }
        }
    }
}

// структура модели заказов для передачи трэдам axum/tokio с помощью разделённого состояния
pub struct OrdersModel {
    postgres_instance: PostgresDB,
    redis_instance: RedisDB,
}

// функции работы с данными о заказе / базами данных
impl OrdersModel {
    // инициализация модели заказов
    pub async fn new(db_config: &DbConfig) -> Result<Self, Box<dyn Error>> {
        // инициализация базы данных
        let postgres_instance = PostgresDB::new(db_config).await?;
        let redis_instance = RedisDB::new(db_config).await?;

        Ok(OrdersModel {
            postgres_instance,
            redis_instance,
        })
    }

    // добавлене нового заказа в базу
    pub async fn insert_order(&self, order: &Order) -> Result<(), ServerError> {
        // запрос к базе данных с тайм-аутом
        let insert_order_result = timeout(Duration::from_secs(1), async {
            self.postgres_instance.insert_order(order).await
        })
        .await;

        // удаление данных о всех запросах из кэша redis
        let redis_del_result = self.redis_instance.del("orders").await;
        match redis_del_result {
            Ok(()) => {
                info!("Данные о всех заказах удалены из кэша Redis")
            }
            Err(err) => return Err(ServerError::PostgresError(err)),
        }

        // обработка ошибок
        match insert_order_result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(err)) => Err(ServerError::PostgresError(err)),
            Err(Elapsed { .. }) => Err(ServerError::TimeoutError(
                format!("Добавление в базу данных: {:?}", order)
            )),
        }
    }

    // добавлене в кэш
    async fn add_to_cache(&self, key: &str, value: &str) -> Result<(), ServerError> {
        // запрос к базе данных redis с тайм-аутом
        let redis_result = timeout(Duration::from_secs(1), async {
            self.redis_instance.set(key, value).await
        })
        .await;

        // обработка ошибок redis
        match redis_result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(err )) => Err(ServerError::RedisError(err)),
            Err(Elapsed { .. }) => Err(ServerError::TimeoutError(
                format!("добавление в кэш key: {:?}, value: {:?}", key, value)
            )),
        }
    }

    // получение всех заказов
    pub async fn get_all_orders(&self) -> Result<Vec<Order>, ServerError> {
        // запрос к базе данных redis с тайм-аутом
        let redis_get_result = timeout(Duration::from_secs(1), async {
            self.redis_instance.get_all_orders().await
        })
        .await;

        // если данные есть в кэшэ - их возрат, обработка ошибок
        match redis_get_result {
            Ok(Ok(Some(orders))) => return Ok(orders),
            Ok(Ok(None)) => {}
            Ok(Err(err)) => return Err(ServerError::RedisError(err)),
            Err(Elapsed { .. }) => {
                warn!("Тайм-аут запроса всех заказов из кэша Redis")
            }
        };

        // запрос к базе данных postgres с тайм-аутом
        let postgres_result = timeout(Duration::from_secs(1), async {
            self.postgres_instance.get_all_orders().await
        })
        .await;

        // если база postgres вернула данные - запись в кэш, в противном случае - обработка ошибок
        match postgres_result {
            Ok(Ok(Some(orders))) => {
                // сериализация
                let orders_str_result = serde_json::to_string(&orders);
                let orders_str = match orders_str_result {
                    Ok(orders_str) => orders_str,
                    Err(_) => return Err(ServerError::SerializationError("Запрос всех заказов".to_string())),
                };

                // запись в кэш
                let redis_set_result = self.add_to_cache("orders", &orders_str).await;
                match redis_set_result {
                    Ok(()) => {
                        info!("Запрос заказов закэширован в базе данных redis");
                        Ok(orders)
                    }
                    Err(err) => Err(err),
                }
            }
            Ok(Ok(None)) => Err(ServerError::NotFound("Получение всех заказов из базы".to_string())),
            Ok(Err(err)) => Err(ServerError::PostgresError(err)),
            Err(Elapsed { .. }) => Err(ServerError::TimeoutError(
                "Получение всех заказов из базы".to_string(),
            )),
        }
    }

    // получение всех заказов по uuid
    pub async fn get_one_order_by_uuid(&self, order_uuid: &Uuid) -> Result<Order, ServerError> {
        let redis_result = timeout(Duration::from_secs(1), async {
            self.redis_instance.get_order(&order_uuid.to_string()).await
        })
        .await;

        // если данные есть в кэшэ - их возрат, обработка ошибок
        match redis_result {
            Ok(Ok(Some(order))) => return Ok(order),
            Ok(Ok(None)) => {}
            Ok(Err(err)) => return Err(ServerError::RedisError(err)),
            Err(Elapsed { .. }) => {
                warn!("Тайм-аут запроса заказа {:?} из кэша Redis", &order_uuid)
            }
        };

        // запрос к базе данных Postgres с тайм-аутом
        let order_result = timeout(Duration::from_secs(1), async {
            self.postgres_instance
                .get_one_order_by_uuid(order_uuid)
                .await
        })
        .await;

        // если база postgres вернула данные - запись в кэш, в противном случае - обработка ошибок
        match order_result {
            Ok(Ok(Some(order))) => {
                // сериаизация
                let order_str_result = serde_json::to_string(&order);
                let order_str = match order_str_result {
                    Ok(order_str) => order_str,
                    Err(_) => return Err(ServerError::SerializationError(format!(
                        "Получение заказа {:?} из базы",
                        order_uuid
                    ))),
                };

                // запись в кэш
                let redis_set_result = self
                    .add_to_cache(&order.order_uid.to_string(), &order_str)
                    .await;
                match redis_set_result {
                    Ok(()) => {
                        info!(
                            "Запрос заказа {} закэширован в базе данных redis",
                            &order.order_uid
                        );
                        Ok(order)
                    }
                    Err(err) => Err(err),
                }
            }
            Ok(Ok(None)) => Err(ServerError::NotFound(format!(
                "Получение заказа {:?} из базы",
                order_uuid
            ))),
            Ok(Err(err)) => Err(ServerError::PostgresError(err)),
            Err(Elapsed { .. }) => Err(ServerError::TimeoutError(format!(
                "Получение заказа {:?} из базы",
                order_uuid
            ))),
        }
    }
}
