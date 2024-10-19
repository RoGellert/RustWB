use crate::data_model::{Message, MessagePayload, RoomPayload, UserPayload};
use crate::errors::ServerError;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use std::sync::Arc;
use uuid::Uuid;

// POST /register - регистрация пользователя
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<(), ServerError> {
    app_state.auth_module.register_user(user_payload).await?;

    Ok(())
}

// POST /login - авторизация пользователя и возврат jwt
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<Json<String>, ServerError> {
    let jwt = app_state.auth_module.login_user(user_payload).await?;

    Ok(Json(jwt))
}

// POST /create_room - создание комнаты и возврат её uuid
pub async fn create_room(
    State(app_state): State<Arc<AppState>>,
    Json(room_payload): Json<RoomPayload>,
) -> Result<Json<Uuid>, ServerError> {
    let room_uuid = app_state.room_module.create_room(room_payload)?;

    Ok(Json(room_uuid))
}

// POST /join/:room_id - присоединение к комнате по uuid
pub async fn join(
    State(app_state): State<Arc<AppState>>,
    Extension(user_login): Extension<String>,
    Path(post_id): Path<Uuid>,
) -> Result<(), ServerError> {
    app_state.room_module.join_room(user_login, post_id)?;

    Ok(())
}

// POST /leave - выход из текущей комнаты
pub async fn leave(
    State(app_state): State<Arc<AppState>>,
    Extension(user_login): Extension<String>,
) -> Result<(), ServerError> {
    app_state.room_module.leave_room(user_login)?;

    Ok(())
}

// POST /send - отправка сообщения в текущую комнату и возврат uuid сообщения
pub async fn send(
    State(app_state): State<Arc<AppState>>,
    Extension(user_login): Extension<String>,
    Json(message_payload): Json<MessagePayload>,
) -> Result<Json<Uuid>, ServerError> {
    let message_uuid = app_state
        .room_module
        .send_message(message_payload, user_login)?;

    Ok(Json(message_uuid))
}

// GET /messages/:room_uuid - получение сообщений из комнаты по uuid
pub async fn messages_by_room_uuid(
    State(app_state): State<Arc<AppState>>,
    Extension(user_login): Extension<String>,
    Path(room_id): Path<Uuid>,
) -> Result<Json<Vec<Message>>, ServerError> {
    let messages = app_state
        .room_module
        .get_messages_by_room_uuid(&room_id, user_login)?;

    Ok(Json(messages))
}

// GET /messages - получение сообщений из комнаты в которой в данный момент находится пользователь
pub async fn messages_by_current_room(
    State(app_state): State<Arc<AppState>>,
    Extension(user_login): Extension<String>,
) -> Result<Json<Vec<Message>>, ServerError> {
    let messages = app_state
        .room_module
        .get_messages_from_curr_room(user_login)?;

    Ok(Json(messages))
}

// GET /user_count - получение количества пользователей
pub async fn user_count(
    State(app_state): State<Arc<AppState>>,
    Extension(user_login): Extension<String>,
) -> Result<Json<usize>, ServerError> {
    let user_count = app_state.room_module.get_user_count(user_login);

    Ok(Json(user_count))
}
