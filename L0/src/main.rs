//! запуск приложения и веб-сервера
pub mod config;
pub mod db {
    pub mod postgres_db;
    pub mod redis_db;
}
pub mod controller;
pub mod model;

use crate::config::DbConfig;
use crate::controller::{get_all_orders, get_order_by_uuid, insert_order};
use crate::model::OrdersModel;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    // ицициализация базы данных
    let db_config = DbConfig::new();
    // инициализация модели заказов
    let orders_model: Arc<OrdersModel> = Arc::new(OrdersModel::new(&db_config).await.unwrap());

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .route("/orders", get(get_all_orders))
        .route("/orders/:order_uuid", get(get_order_by_uuid))
        .route("/orders", post(insert_order))
        .with_state(orders_model);

    // старт сервера на порту 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Сервер AXUM готов принимать запросы на порту 3000");
    axum::serve(listener, app).await.unwrap();
}
