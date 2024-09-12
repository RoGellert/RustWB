//! функции поведения эндпоинтов
use crate::model::{Order, OrdersModel, ServerError};
use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;
use uuid::Uuid;

// GET /orders - получение всех заказов из базы данных
pub async fn get_all_orders(
    State(orders_model): State<Arc<OrdersModel>>,
) -> Result<Json<Vec<Order>>, ServerError> {
    // получение всех заказов из базы данных
    let query_response = orders_model.get_all_orders().await?;

    Ok(Json(query_response))
}

// GET /orders/:order_uuid - получение всех заказов из базы данных по order_uuid
pub async fn get_order_by_uuid(
    State(orders_model): State<Arc<OrdersModel>>,
    Path(order_uuid): Path<Uuid>,
) -> Result<Json<Order>, ServerError> {
    // получение одного заказа из базы данных по uuid
    let query_response = orders_model.get_one_order_by_uuid(&order_uuid).await?;

    Ok(Json(query_response))
}

// POST /orders - добавление одного заказа (JSON заказа в теле запроса)
pub async fn insert_order(
    State(orders_model): State<Arc<OrdersModel>>,
    Json(order): Json<Order>,
) -> Result<Json<Order>, ServerError> {
    // запись заказа в базу данных
    orders_model.insert_order(&order).await?;

    Ok(Json(order))
}
