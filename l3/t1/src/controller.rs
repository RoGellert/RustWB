use crate::data_types::{User, UserPayload};
use crate::errors::ServerError;
use crate::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use std::sync::Arc;

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
) -> Result<Json<String>, ServerError> {
    // получение всех заказов из базы данных
    let jwt = app_state.auth_module.login_user(user_payload).await?;

    Ok(Json(jwt))
}

pub async fn hello(Extension(user): Extension<User>) -> Result<Json<User>, ServerError> {
    Ok(Json(user))
}
