use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;

// потенциальные ошибки
pub enum ServerError {
    BusinessLogic(String),
    NotFound(String),
    Lock(String),
    Unauthorised(String),
    PasswordHashGeneration(String),
    Jwt(String),
    Validation(String),
    SendingToThread(String),
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::Lock(text) => {
                error!("Ошибка блокирования Mutex или RwLock: {:?}", text);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка блокирования Mutex или RwLock: {:?}", text),
                )
                    .into_response()
            }
            ServerError::SendingToThread(text) => {
                error!(
                    "Не удалось отправить сообщение через канал отправки: {:?}",
                    text
                );
                (
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Не удалось отправить сообщение через канал отправки: {:?}",
                        text
                    ),
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
            ServerError::Validation(text) => {
                error!("Ошибка валидации входных данных: {:?}", text);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Ошибка валидации входных данных: {:?}", text),
                )
                    .into_response()
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
