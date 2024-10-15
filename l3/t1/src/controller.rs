use crate::data_types::{Post, PostPayload, UserPayload};
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

// POST /posts - создание поста
pub async fn create_post(
    State(app_state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<Uuid>,
    Json(post_payload): Json<PostPayload>,
) -> Result<(), ServerError> {
    app_state
        .post_module
        .create_post(user_uuid, post_payload)
        .await?;

    Ok(())
}

// GET /posts/:post_id - получение поста по Uuid
pub async fn get_post_by_uuid(
    State(app_state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<Post>, ServerError> {
    let post = app_state.post_module.get_post_by_uuid(post_id).await?;

    Ok(Json(post))
}

// GET /user/posts - получение всех постов пользователя
pub async fn get_all_posts_by_user_uuid(
    Extension(user_uuid): Extension<Uuid>,
    State(app_state): State<Arc<AppState>>
) -> Result<Json<Vec<Post>>, ServerError> {
    let posts = app_state.post_module.get_all_posts_by_user_uuid(user_uuid).await?;

    Ok(Json(posts))
}

// DELETE /posts/:post_id - удаление поста по Uuid
pub async fn delete_post_by_uuid(
    State(app_state): State<Arc<AppState>>,
    Path(post_id): Path<Uuid>
) -> Result<(), ServerError> {
    app_state.post_module.delete_post_by_uuid(post_id).await?;

    Ok(())
}

// POST /posts/:post_id/likes - лайк поста по Uuid
pub async fn like_post(
    State(app_state): State<Arc<AppState>>,
    Extension(user_uuid): Extension<Uuid>,
    Path(post_id): Path<Uuid>
) -> Result<(), ServerError> {
    app_state.post_module.insert_like(user_uuid, post_id).await?;

    Ok(())
}
