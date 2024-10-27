//! функции поведения эндпоинтов
use crate::errors::ServerError;
use crate::modules::event_manager::EventPayload;
use crate::AppState;
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::Response;
use axum::Json;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

// POST /subscriptions/:user_uuid/:event_type - подписка пользователя на событие определённого типа
pub async fn subscribe(
    State(app_state): State<Arc<AppState>>,
    Path((user_uuid, event_type)): Path<(Uuid, String)>,
) -> Result<(), ServerError> {
    // получение всех заказов из базы данных
    app_state
        .subscription_manager
        .subscribe(user_uuid, event_type)
        .await?;

    Ok(())
}

// GET /subscriptions/:user_uuid - получение всех подписок пользователя по uuid
pub async fn get_subscriptions_by_user_uuid(
    State(app_state): State<Arc<AppState>>,
    Path(user_uuid): Path<Uuid>,
) -> Result<Json<Vec<String>>, ServerError> {
    // получение всех заказов из базы данных
    let subscriptions = app_state
        .subscription_manager
        .get_all_subscriptions_by_user_uuid(user_uuid)
        .await?;

    Ok(Json(subscriptions))
}

// POST /subscriptions/:user_uuid/:event_type - подписка пользователя на событие определённого типа
pub async fn add_event(
    State(app_state): State<Arc<AppState>>,
    Json(event): Json<EventPayload>,
) -> Result<(), ServerError> {
    // получение всех заказов из базы данных
    app_state.event_manager.add_event(event).await?;

    Ok(())
}

// GET - получение ивентов на которые подписан пользователь по uuid
pub async fn get_events_by_user_uuid(
    State(app_state): State<Arc<AppState>>,
    Path(user_uuid): Path<Uuid>,
) -> Result<Json<Vec<String>>, ServerError> {
    // получение всех ивентов из базы данных
    let events = app_state
        .event_manager
        .get_all_events_by_user_uuid(user_uuid)
        .await?;

    Ok(Json(events))
}

// хэндлер WebSocket
pub async fn notification_socket_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<Arc<AppState>>,
    Path(user_uuid): Path<Uuid>,
) -> Response {
    let state_clone = Arc::clone(&app_state);
    ws.on_upgrade(move |socket| async move {
        if let Err(err) = state_clone
            .notification_module
            .notification_socket(socket, user_uuid)
            .await
        {
            error!("Ошибка подключения через WebSocket {:?}", err)
        }
    })
}
