use crate::config::Config;
use crate::controller::{create_post, delete_post_by_uuid, get_all_posts_by_user_uuid, get_post_by_uuid, like_post, login, register};
use crate::modules::auth_module::{jwt_protected, AuthModule};
use crate::modules::post_module::PostModule;
use crate::modules::user_module::UserModule;
use crate::pg_db::PostgresDB;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post};
use axum::Router;
use std::sync::Arc;
use tracing::{info, Level};

pub mod modules {
    pub mod auth_module;
    pub mod post_module;
    pub mod user_module;
}
mod config;
mod controller;
mod data_types;
mod errors;
mod pg_db;

pub struct AppState {
    pub user_module: Arc<UserModule>,
    pub auth_module: AuthModule,
    pub post_module: PostModule,
}

impl AppState {
    pub fn new(user_module: Arc<UserModule>, auth_module: AuthModule, post_module: PostModule) -> Self {
        AppState {
            user_module,
            auth_module,
            post_module,
        }
    }
}

#[tokio::main]
async fn main() {
    // ицициализация конфига
    let config = Config::new();
    // инициализация базы данных
    let postgres_db = Arc::new(
        PostgresDB::new(config.db_config)
            .await
            .expect("Ошибка подключения к базе данных"),
    );
    // инициализация модуля пользователей
    let user_module = Arc::new(UserModule::new(Arc::clone(&postgres_db)));
    // инициализация модуля авторизации
    let auth_module = AuthModule::new(config.auth_config, Arc::clone(&user_module));
    // инициализация модуля постов
    let post_module = PostModule::new(Arc::clone(&postgres_db));

    // разделённое состояние
    let app_state: Arc<AppState> = Arc::new(AppState::new(Arc::clone(&user_module), auth_module, post_module));

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .route(
            "/posts",
            post(create_post),
        )
        .route(
            "/user/posts",
            get(get_all_posts_by_user_uuid),
        )
        .route(
            "/posts/:post_id",
            get(get_post_by_uuid),
        )
        .route(
            "/posts/:post_id",
            delete(delete_post_by_uuid),
        )
        .route(
            "/posts/:post_id/likes",
            post(like_post),
        )
        .layer(from_fn_with_state(Arc::clone(&app_state), jwt_protected))
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(app_state);

    // старт сервера на порту 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("не удалось создать TcpListener");
    info!("Сервер AXUM готов принимать запросы на порту 3000");
    axum::serve(listener, app).await.expect("не удалось запустить сервер AXUM");
}
