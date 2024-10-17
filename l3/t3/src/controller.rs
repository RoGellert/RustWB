use crate::data_types::{Post, PostPayload, UserPayload};
use crate::errors::ServerError;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use std::sync::Arc;
use crate::data_model::UserPayload;

// POST /register - регистрация пользователя
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<(), ServerError> {
    // получение всех заказов из базы данных
    app_state.auth_module.register_user(user_payload).await?;

    Ok(())
}

// POST /login - авторизация пользователя
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(user_payload): Json<UserPayload>,
) -> Result<Json<String>, ServerError> {
    // получение всех заказов из базы данных
    let jwt = app_state.auth_module.login_user(user_payload).await?;

    Ok(Json(jwt))
}
