//! запуск приложения и веб-сервера
mod config;
mod modules {
    pub mod event_manager;
    pub mod notification_module;
    pub mod subscription_manager;
}
mod controller;
mod errors;
pub mod redis_db;

use crate::config::RedisConfig;
use crate::controller::{
    add_event, get_events_by_user_uuid, get_subscriptions_by_user_uuid,
    notification_socket_handler, subscribe,
};
use crate::modules::event_manager::EventManager;
use crate::modules::notification_module::NotificationModule;
use crate::modules::subscription_manager::SubscriptionManager;
use crate::redis_db::RedisDB;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tracing::{info, Level};

// разделённое состояие
pub struct AppState {
    // мэнеджер подписок
    subscription_manager: Arc<SubscriptionManager>,
    // мэнеджер событий
    event_manager: EventManager,
    // модуль нотификаций
    notification_module: NotificationModule,
}

impl AppState {
    pub fn new(
        subscription_manager: Arc<SubscriptionManager>,
        event_manager: EventManager,
        notification_module: NotificationModule,
    ) -> Self {
        AppState {
            subscription_manager,
            event_manager,
            notification_module,
        }
    }
}

#[tokio::main]
async fn main() {
    // ицициализация конфига базы данных
    let db_config = RedisConfig::new();
    // ицициализация базы данных
    let redis_db = Arc::new(
        RedisDB::new(&db_config)
            .await
            .expect("не удалось подключиться к базе данных"),
    );

    // создание мэнеджеров и модулей
    let subscription_manager = Arc::new(SubscriptionManager::new(Arc::clone(&redis_db)));
    let notification_module =
        NotificationModule::new(Arc::clone(&redis_db), Arc::clone(&subscription_manager));
    let event_manager = EventManager::new(Arc::clone(&redis_db), Arc::clone(&subscription_manager));
    // инициализация разделяемого состояния
    let app_state = Arc::new(AppState::new(
        subscription_manager,
        event_manager,
        notification_module,
    ));

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .route("/subscriptions/:user_uuid/:event_type", post(subscribe))
        .route(
            "/subscriptions/:user_uuid",
            get(get_subscriptions_by_user_uuid),
        )
        .route("/events", post(add_event))
        .route("/events/:user_uuid", get(get_events_by_user_uuid))
        .route(
            "/notifications/:user_uuid",
            get(notification_socket_handler),
        )
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
