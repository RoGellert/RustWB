use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use tracing::error;

// потенциальные ошибки
pub enum ServerError {
    BusinessLogic(String),
    NotFound(String),
    Postgres(Box<dyn Error>),
    Serialization(String),
    Unauthorised(String),
    PasswordHashGeneration(String),
    Jwt(String),
    Validation(String),
    Unknown,
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::NotFound(text) => {
                error!("Данные по запросу не найдены: {:?}", text);
                (
                    StatusCode::NOT_FOUND,
                    format!("Данные по запросу не найдены: {:?}", text),
                )
                    .into_response()
            }
            ServerError::Validation(text) => {
                error!("Ошибка валидации входных данных: {:?}", text);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Ошибка валидации входных данных: {:?}", text),
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
                error!("Ошибка авторизации запроса: {:?}", text);
                (
                    StatusCode::UNAUTHORIZED,
                    format!("Ошибка авторизации запроса: {:?}", text),
                )
                    .into_response()
            }
            ServerError::PasswordHashGeneration(text) => {
                error!("Ошибка пароля: {:?}", text);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка пароля: {:?}", text),
                )
                    .into_response()
            }
            ServerError::Jwt(text) => {
                error!("Ошибка jwt: {:?}", text);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка генерации jwt: {:?}", text),
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
        }
    }
}
