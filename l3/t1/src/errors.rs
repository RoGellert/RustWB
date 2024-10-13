use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use tracing::{error, warn};

// потенциальные ошибки
pub enum ServerError {
    BusinessLogic(String),
    NotFound(String),
    Postgres(Box<dyn Error>),
    Serialization(String),
    Unauthorised(String),
    Password(String),
    Jwt(String),
    Unknown,
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::NotFound(text) => {
                warn!("Данные по запросу не найдены: {:?}", text);
                (
                    StatusCode::NOT_FOUND,
                    format!("Данные по запросу не найдены: {:?}", text),
                )
                    .into_response()
            }
            ServerError::Postgres(err) => {
                error!("Ошибка базы данных Postgres {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка базы данных Postgres {:?}", err),
                )
                    .into_response()
            }
            ServerError::Serialization(text) => {
                error!("Ошибка сериализации в запросе {:?}", text);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка десериализации в запросе {:?}", text),
                )
                    .into_response()
            }
            ServerError::Unknown => {
                error!("Неизвестная ошибка");
                (StatusCode::INTERNAL_SERVER_ERROR, "Неизвестная ошибка").into_response()
            }
            ServerError::Unauthorised(text) => {
                error!("Неавторизованный запрос: {:?}", text);
                (
                    StatusCode::UNAUTHORIZED,
                    format!("Неавторизованный запрос: {:?}", text),
                )
                    .into_response()
            }
            ServerError::Password(text) => {
                error!("Ошибка пароля: {:?}", text);
                (
                    StatusCode::UNAUTHORIZED,
                    format!("Ошибка пароля: {:?}", text),
                )
                    .into_response()
            }
            ServerError::Jwt(text) => {
                error!("Ошибка jwt: {:?}", text);
                (StatusCode::UNAUTHORIZED, format!("Ошибка jwt: {:?}", text)).into_response()
            }
            ServerError::BusinessLogic(text) => {
                error!("Ошибка бизнес-логики: {:?}", text);
                (
                    StatusCode::UNAUTHORIZED,
                    format!("Ошибка бизнес-логики: {:?}", text),
                )
                    .into_response()
            }
        }
    }
}
