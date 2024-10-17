use std::sync::Arc;
use axum::Router;
use tracing::{info, Level};
use crate::app_data::AppData;
use crate::config::AuthConfig;
use crate::modules::auth_module::AuthModule;

mod app_data;
mod modules {
    pub mod auth_module;

}
mod data_model;
mod errors;
mod config;
mod controller;

pub struct AppState {
    auth_module: AuthModule,
}

impl AppState {
    pub fn new(auth_module: AuthModule) -> Self {
        AppState {
            auth_module
        }
    }
}

#[tokio::main]
async fn main() {
    // ицициализация конфига
    let auth_config = AuthConfig::new();

    // инициализация состояния данных
    let app_data = Arc::new(AppData::new());
    // инициализация модуля авторизации
    let auth_module = AuthModule::new( Arc::clone(&app_data), auth_config);

    // разделённое состояние
    let app_state = Arc::new(AppState::new(auth_module));

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .with_state(app_state);

    // старт сервера на порту 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("не удалось создать TcpListener");
    info!("Сервер AXUM готов принимать запросы на порту 3000");
    axum::serve(listener, app).await.expect("не удалось запустить сервер AXUM");
}
