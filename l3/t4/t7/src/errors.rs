use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use tracing::error;

// потенциальные ошибки
pub enum ServerError {
    Redis(Box<dyn Error>),
    NotFound(String),
    BusinessLogic(String),
    Serialisation(String),
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::Redis(err) => {
                error!("Ошибка базы данных Redis {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка базы данных Redis {:?}", err),
                )
                    .into_response()
            }
            ServerError::NotFound(text) => {
                error!("Данные по запросу не найдены: {:?}", text);
                (
                    StatusCode::NOT_FOUND,
                    format!("Данные по запросу не найдены: {:?}", text),
                )
                    .into_response()
            }
            ServerError::BusinessLogic(text) => {
                error!("Ошибка бизнес-логики: {:?}", text);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Ошибка бизнес-логики: {:?}", text),
                )
                    .into_response()
            }
            ServerError::Serialisation(text) => {
                error!("Ошибка сериализации: {:?}", text);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка сериализации: {:?}", text),
                )
                    .into_response()
            }
        }
    }
}
