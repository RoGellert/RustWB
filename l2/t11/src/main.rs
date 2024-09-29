use crate::controller::{create_event, delete_event, events_for_day, events_for_month, events_for_week, update_event};
use crate::model::{Event, EventModel};
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tracing::{info, Level};

mod controller;
mod model;

#[tokio::main]
async fn main() {
    // инициализация модели заказов
    let event_model: Arc<EventModel> = Arc::new(EventModel::new());

    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // конфигурация энд-поинтов и общих ресурсов
    let app = Router::new()
        .route("/create_event", post(create_event))
        .route("/update_event", post(update_event))
        .route("/delete_event/:event_id", post(delete_event))
        .route("/events_for_day/:date", get(events_for_day))
        .route("/events_for_week/:date", get(events_for_week))
        .route("/events_for_month/:date", get(events_for_month))
        .with_state(event_model);

    // старт сервера на порту 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Сервер AXUM готов принимать запросы на порту 3000");
    axum::serve(listener, app).await.unwrap();
}
