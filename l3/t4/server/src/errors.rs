use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use tracing::error;

// потенциальные ошибки
pub enum ServerError {
    Postgres(Box<dyn Error>),
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::Postgres(err) => {
                error!("Ошибка базы данных Postgres {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка базы данных Postgres {:?}", err),
                )
                    .into_response()
            }
        }
    }
}
