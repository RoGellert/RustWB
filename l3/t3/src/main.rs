use crate::app_data::AppData;
use crate::config::AuthConfig;
use crate::controller::{
    create_room, join, leave, login, messages_by_current_room, messages_by_room_uuid, register,
    send, user_count,
};
use crate::modules::auth_module::{jwt_protected, AuthModule};
use crate::modules::room_module::RoomModule;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tracing::{info, Level};

mod app_data;
mod modules {
    pub mod auth_module;
    pub mod room_module;
}
mod config;
mod controller;
mod data_model;
mod errors;

pub struct AppState {
    // модуль аутентификации
    auth_module: AuthModule,
    // модуль работы с комнатами/сообщениями
    room_module: RoomModule,
}

impl AppState {
    pub fn new(auth_module: AuthModule, room_module: RoomModule) -> Self {
        AppState {
            auth_module,
            room_module,
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
    let auth_module = AuthModule::new(Arc::clone(&app_data), auth_config);
    // инициализация модуля комнат
    let room_module = RoomModule::new(Arc::clone(&app_data));

    // разделённое состояние
    let app_state = Arc::new(AppState::new(auth_module, room_module));

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .route("/create_room", post(create_room))
        .route("/join/:room_id", post(join))
        .route("/leave", post(leave))
        .route("/send", post(send))
        .route("/messages", get(messages_by_current_room))
        .route("/messages/:room_id", get(messages_by_room_uuid))
        .route("/user_count", get(user_count))
        .layer(from_fn_with_state(Arc::clone(&app_state), jwt_protected))
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(app_state);

    // старт сервера на порту 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("не удалось создать TcpListener");
    info!("Сервер AXUM готов принимать запросы на порту 3000");
    axum::serve(listener, app)
        .await
        .expect("не удалось запустить сервер AXUM");
}
