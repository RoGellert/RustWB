use crate::errors::ServerError;
use crate::modules::user_module::{User, UserPayload};
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Json};
use std::sync::Arc;
use crate::modules::product_module::{Product, ProductPayload};

// POST /users - добавление пользователя
pub async fn add_user(
    State(app_state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<(), ServerError> {
    app_state.user_module.add_user(user).await?;

    Ok(())
}

// PUT /users/:user_id - обновление данных пользователя
pub async fn update_user(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
    Json(user_payload): Json<UserPayload>
) -> Result<(), ServerError> {
    app_state.user_module.update_user(user_id, user_payload).await?;

    Ok(())
}

// DELETE /users/:user_id - удаление данных пользователя
pub async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Path(user_id): Path<i32>
) -> Result<(), ServerError> {
    app_state.user_module.delete_user(user_id).await?;

    Ok(())
}

// POST /products - добавление продукта
pub async fn add_product(
    State(app_state): State<Arc<AppState>>,
    Json(product): Json<Product>,
) -> Result<(), ServerError> {
    app_state.product_module.add_product(product).await?;

    Ok(())
}

// PUT /products/:product_id - обновление данных продукта
pub async fn update_product(
    State(app_state): State<Arc<AppState>>,
    Path(product_id): Path<i32>,
    Json(product_payload): Json<ProductPayload>,
) -> Result<(), ServerError> {
    app_state.product_module.update_product(product_id, product_payload).await?;

    Ok(())
}

// DELETE /products/:product_id - удаление данных продукта
pub async fn delete_product(
    State(app_state): State<Arc<AppState>>,
    Path(product_id): Path<i32>
) -> Result<(), ServerError> {
    app_state.product_module.delete_product(product_id).await?;

    Ok(())
}

