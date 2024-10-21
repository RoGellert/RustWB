use crate::config::DbConfig;
use crate::controller::add_user;
use crate::modules::product_module::ProductModule;
use crate::modules::user_module::UserModule;
use crate::pg_db::PostgresDB;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;
use tracing::{info, Level};

mod config;
mod controller;
mod errors;
mod pg_db;

mod modules {
    pub mod product_module;
    pub mod user_module;
}

struct AppState {
    user_module: UserModule,
    product_module: ProductModule,
}

impl AppState {
    pub fn new(user_module: UserModule, product_module: ProductModule) -> Self {
        AppState {
            user_module,
            product_module,
        }
    }
}

#[tokio::main]
async fn main() {
    // ицициализация конфига
    let db_config = DbConfig::new();
    // инициализация базы данных
    let postgres_db = Arc::new(
        PostgresDB::new(db_config)
            .await
            .expect("Ошибка подключения к базе данных"),
    );
    // инициализация модуля пользователей
    let user_module = UserModule::new(Arc::clone(&postgres_db));
    // инициализация модуля продуктов
    let product_module = ProductModule::new(Arc::clone(&postgres_db));

    // разделённое состояние
    let app_state: Arc<AppState> = Arc::new(AppState::new(user_module, product_module));

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .route("/users", post(add_user))
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
