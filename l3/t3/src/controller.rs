use crate::errors::ServerError;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{debug_handler, Json};
use std::sync::Arc;
use crate::data_model::UserPayload;

// POST /register - регистрация пользователя
#[debug_handler]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<(), ServerError> {
    app_state.auth_module.register_user(user_payload).await?;

    Ok(())
}

// POST /login - авторизация пользователя
#[debug_handler]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<Json<String>, ServerError> {
    let jwt = app_state.auth_module.login_user(user_payload).await?;

    Ok(Json(jwt))
}
