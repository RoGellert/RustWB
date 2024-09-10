use crate::model::{Order, OrdersModel};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use tracing::warn;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

pub enum ServerError {
    NotFound,
    InvalidInputError,
    DbError(Box<dyn Error>),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::NotFound => (StatusCode::NOT_FOUND, "Данные не найдены").into_response(),
            ServerError::DbError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Внутренняя ошибка сервера {:?}", e),
            )
                .into_response(),
            ServerError::InvalidInputError => {
                (StatusCode::BAD_REQUEST, "Неккорректые данные в запросе").into_response()
            }
        }
    }
}

pub async fn get_all_orders(
    State(orders_model): State<Arc<OrdersModel>>,
) -> Result<Json<Vec<Order>>, ServerError> {
    // получение всех заказов из базы данных
    let query_response = orders_model.get_all_orders().await;

    // обработка ошибок
    match query_response {

        Ok(Some(orders)) => Ok(Json(orders)),
        Ok(None) => {
            warn!("Заказы отсутствуют в базе данных");
            Err(ServerError::NotFound)
        }
        Err(err) => {
            warn!("Ошибка базы данных: {err}");
            Err(ServerError::DbError(err))
        }
    }
}

pub async fn get_order_by_uuid(
    State(orders_model): State<Arc<OrdersModel>>,
    Path(order_uuid): Path<Uuid>,
) -> Result<Json<Order>, ServerError> {
    // получение одного заказа из базы данных по uuid
    let query_response = orders_model.get_one_order_by_uuid(&order_uuid).await;

    // обработка ошибок
    match query_response {
        Ok(Some(order)) => Ok(Json(order)),
        Ok(None) => {
            warn!("Заказ c uuid {order_uuid} отсутствуют в базе данных");
            Err(ServerError::NotFound)
        }
        Err(err) => {
            warn!("Ошибка базы данных: {err}");
            Err(ServerError::DbError(err))
        }
    }
}

pub async fn insert_order(
    State(orders_model): State<Arc<OrdersModel>>,
    Json(order): Json<Order>,
) -> Result<Json<Order>, ServerError> {
    // запись заказа в базу данных
    let query_response = orders_model.insert_order(&order).await;

    // обработка ошибок
    match query_response {
        Ok(()) => Ok(Json(order)),
        Err(err) => {
            warn!("Ошибка базы данных: {err}");
            Err(ServerError::DbError(err))
        }
    }
}
