use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use tokio::sync::Mutex;
use tracing::error;

// пример валидации
fn validate_event(event: &Event) -> Result<(), String> {
    if event.date < NaiveDate::from_ymd_opt(2023, 1, 1).unwrap() {
        return Err("минимально возможная дата: 2023-1-1".to_string());
    }

    if event.name.len() > 25 || event.name.len() < 5 {
        return Err("длина имени ивента должна быть между 5 и 25".to_string());
    }

    if event.event_id == 0 {
        return Err("id ивента не может быть равно 0".to_string());
    }

    if event.user_id == 0 {
        return Err("id пользователя не может быть равно 0".to_string());
    }

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub event_id: u32,
    pub name: String,
    pub user_id: u32,
    pub date: NaiveDate,
}

// потенциальные ошибки
pub enum ServerError {
    BusinessLogic(String),
    InvalidInput(String),
    Internal(String),
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::BusinessLogic(err) => {
                error!("Ошибка бизнес логики: {:?}", err);
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!("Ошибка бизнес логики: {:?}", err),
                )
                    .into_response()
            }
            ServerError::InvalidInput(err) => {
                error!("Ошибка входных данных {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Ошибка входных данных {:?}", err),
                )
                    .into_response()
            }
            ServerError::Internal(err) => {
                error!("Ошибка сервера {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка сервера {:?}", err),
                )
                    .into_response()
            }
        }
    }
}

pub struct EventModel {
    // для хранения ивентов
    event_map: Mutex<HashMap<u32, Event>>,
}

impl EventModel {
    // инициализация модели событий
    pub fn new() -> Self {
        // инициализация переменных разделяемого состояния
        let event_map: Mutex<HashMap<u32, Event>> = Mutex::new(HashMap::new());

        Self { event_map }
    }

    pub async fn create_event(&self, event: Event) -> Result<(), ServerError> {
        if let Err(message) = validate_event(&event) {
            return Err(ServerError::InvalidInput(format!(
                "Неверный формат входных данных: {}",
                message
            )));
        }

        let event_id = event.event_id;

        let mut event_map = self.event_map.lock().await;

        if event_map.contains_key(&event_id) {
            return Err(ServerError::BusinessLogic(format!(
                "Ивент с id {} уже присутсвует в памяти",
                event_id
            )));
        }

        event_map.insert(event_id, event);

        Ok(())
    }

    pub async fn update_event(&self, event: Event) -> Result<(), ServerError> {
        if let Err(message) = validate_event(&event) {
            return Err(ServerError::InvalidInput(format!(
                "Неверный формат входных данных: {}",
                message
            )));
        }

        let event_id = event.event_id;

        let mut event_map = self.event_map.lock().await;
        if !event_map.contains_key(&event_id) {
            return Err(ServerError::BusinessLogic(format!(
                "Ивент с id {} отсутствует в памяти",
                event_id
            )));
        }

        event_map.insert(event_id, event);

        Ok(())
    }

    pub async fn delete_event(&self, event_id: u32) -> Result<(), ServerError> {
        let mut event_map = self.event_map.lock().await;
        if !event_map.contains_key(&event_id) {
            return Err(ServerError::BusinessLogic(format!(
                "Ивент с id {} отсутствует в памяти",
                event_id
            )));
        }

        event_map.remove(&event_id);

        Ok(())
    }

    pub async fn events_for_day(&self, required_date: NaiveDate) -> Result<Json<Vec<Event>>, ServerError> {
        let event_map = self.event_map.lock().await;

        let events_from_memory: Vec<Event> = event_map
            .values()
            .filter(|event| event.date == required_date)
            .cloned()
            .collect();

        Ok(Json(events_from_memory))
    }

    pub async fn events_for_week(&self, required_date: NaiveDate) -> Result<Json<Vec<Event>>, ServerError> {
        let event_map = self.event_map.lock().await;

        let events_from_memory: Vec<Event> = event_map
            .values()
            .filter(|event| event.date.week(Weekday::Mon).first_day() == required_date.week(Weekday::Mon).first_day())
            .cloned()
            .collect();

        Ok(Json(events_from_memory))
    }

    pub async fn events_for_month(&self, required_date: NaiveDate) -> Result<Json<Vec<Event>>, ServerError> {
        let event_map = self.event_map.lock().await;

        let events_from_memory: Vec<Event> = event_map
            .values()
            .filter(|event| event.date.year() == required_date.year() && event.date.month() == required_date.month())
            .cloned()
            .collect();

        Ok(Json(events_from_memory))
    }
}
