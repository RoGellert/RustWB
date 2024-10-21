use crate::errors::ServerError;
use crate::modules::user_module::User;
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use std::sync::Arc;

// POST /add_user - добавление пользователя
pub async fn add_user(
    State(app_state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<(), ServerError> {
    app_state.user_module.add_user(user).await?;

    Ok(())
}
