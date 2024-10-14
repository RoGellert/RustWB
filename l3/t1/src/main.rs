use crate::config::Config;
use crate::controller::{hello, login, register};
use crate::modules::auth_module::{jwt_protected, AuthModule};
use crate::modules::user_module::UserModule;
use crate::pg_db::PostgresDB;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
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

#[derive(Clone)]
pub struct AppState {
    pub auth_module: AuthModule,
}

impl AppState {
    pub fn new(auth_module: AuthModule) -> Self {
        AppState { auth_module }
    }
}

#[tokio::main]
async fn main() {
    // ицициализация базы данных
    let config = Config::new();
    // инициализация базы данных
    let postgres_db = Arc::new(
        PostgresDB::new(&config.db_config)
            .await
            .expect("Ошибка подключения к базе данных"),
    );
    // инициализация модуля пользователей
    let user_module = UserModule::new(Arc::clone(&postgres_db));
    // инициализация модуля авторизации
    let auth_module = AuthModule::new(config.auth_config, user_module);

    // разделённое состояние
    let app_state: Arc<AppState> = Arc::new(AppState::new(auth_module));

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route(
            "/hello",
            get(hello).layer(from_fn_with_state(Arc::clone(&app_state), jwt_protected)),
        )
        .with_state(app_state);

    // старт сервера на порту 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("не удалось запустить сервер AXUM");
    info!("Сервер AXUM готов принимать запросы на порту 3000");
    axum::serve(listener, app).await.unwrap();
}
