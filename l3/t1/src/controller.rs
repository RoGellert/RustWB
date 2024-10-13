use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::AppState;
use crate::data_types::UserPayload;
use crate::errors::ServerError;

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<(), ServerError> {
    // получение всех заказов из базы данных
    app_state.auth_module.register_user(user_payload).await?;

    Ok(())
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<String, ServerError> {
    // получение всех заказов из базы данных
    let jwt = app_state.auth_module.login_user(user_payload).await?;

    Ok(jwt)
}