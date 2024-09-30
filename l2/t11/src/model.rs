use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;
use tracing::{error, info};

// структура ивента
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub event_id: u32,
    pub name: String,
    pub user_id: u32,
    pub date: NaiveDate,
}

impl Event {
    // пример валидации
    pub fn is_valid(&self) -> Result<(), String> {
        if self.date < NaiveDate::from_ymd_opt(2023, 1, 1).unwrap() {
            return Err("минимально возможная дата: 2023-1-1".to_string());
        }

        if self.name.len() > 25 || self.name.len() < 5 {
            return Err("длина имени ивента должна быть между 5 и 25".to_string());
        }

        if self.event_id == 0 {
            return Err("id ивента не может быть равно 0".to_string());
        }

        if self.user_id == 0 {
            return Err("id пользователя не может быть равно 0".to_string());
        }

        Ok(())
    }
}

// структура результата для возврата
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EventResult<T> {
    result: T,
}

impl<T> EventResult<T> {
    pub fn new(event: T) -> Self {
        Self { result: event }
    }
}

// потенциальные ошибки
pub enum ServerError {
    BusinessLogic(String),
    InvalidInput(String),
    // Internal(String),
}

// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::BusinessLogic(err) => {
                error!("Ошибка бизнес логики: {:?}", err);
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    format!(r#"{{"error": "oшибка бизнес логики: {:?}}}"#, err),
                )
                    .into_response()
            }
            ServerError::InvalidInput(err) => {
                error!("Ошибка входных данных {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    format!(r#"{{"error": "Ошибка входных данных: {:?}}}"#, err),
                )
                    .into_response()
            }
            // ServerError::Internal(err) => {
            //     error!("Ошибка сервера {:?}", err);
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         format!(r#"{{"Ошибка сервера: {:?}}}"#, err),
            //     )
            //         .into_response()
            // }
        }
    }
}

// модель для использования через разделённое состояние
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

    // создание ивента
    pub async fn create_event(&self, event: Event) -> Result<EventResult<Event>, ServerError> {
        // валидация
        if let Err(message) = event.is_valid() {
            return Err(ServerError::InvalidInput(format!(
                "Неверный формат входных данных: {}",
                message
            )));
        }

        let event_id = event.event_id;

        let mut event_map = self.event_map.lock().await;

        // если ивент уже присутствует в данных
        if event_map.contains_key(&event_id) {
            return Err(ServerError::BusinessLogic(format!(
                "Ивент с id {} уже присутсвует в памяти",
                event_id
            )));
        }

        // добавление ивента
        event_map.insert(event_id, event);

        info!("Ивент с id: {} добавлен в память", event_id);

        Ok(EventResult::new(event_map[&event_id].clone()))
    }

    // обновление ивента
    pub async fn update_event(&self, event: Event) -> Result<EventResult<Event>, ServerError> {
        // валидация
        if let Err(message) = event.is_valid() {
            return Err(ServerError::InvalidInput(format!(
                "Неверный формат входных данных: {}",
                message
            )));
        }

        let event_id = event.event_id;

        // если ивент отсутствует в данных
        let mut event_map = self.event_map.lock().await;
        if !event_map.contains_key(&event_id) {
            return Err(ServerError::BusinessLogic(format!(
                "Ивент с id {} отсутствует в памяти",
                event_id
            )));
        }

        // обновление ивента
        event_map.insert(event_id, event);

        info!("Ивент с id: {} изменен в памяти", event_id);

        Ok(EventResult::new(event_map[&event_id].clone()))
    }

    // удаление ивента из памяти
    pub async fn delete_event(&self, event_id: u32) -> Result<EventResult<Event>, ServerError> {
        let mut event_map = self.event_map.lock().await;

        // удаление и проверка на факт отсутствия
        let event = match event_map.remove(&event_id) {
            Some(event) => event,
            None => {
                return Err(ServerError::BusinessLogic(format!(
                    "Ивент с id {} отсутствует в памяти",
                    event_id
                )));
            }
        };

        info!("Ивент с id: {} удален из памяти", event_id);

        Ok(EventResult::new(event))
    }

    // получение ивентов в дне по дате
    pub async fn events_for_day(
        &self,
        required_date: NaiveDate,
    ) -> Result<EventResult<Vec<Event>>, ServerError> {
        let event_map = self.event_map.lock().await;

        // получение ивентов в дне по дате
        let events: Vec<Event> = event_map
            .values()
            .filter(|event| event.date == required_date)
            .cloned()
            .collect();

        info!("Ивенты с датой: {} получены из памяти", required_date.to_string());

        Ok(EventResult::new(events))
    }

    // получение ивентов в неделе по дате
    pub async fn events_for_week(
        &self,
        required_date: NaiveDate,
    ) -> Result<EventResult<Vec<Event>>, ServerError> {
        let event_map = self.event_map.lock().await;

        // получение ивентов в неделе по дате
        let events: Vec<Event> = event_map
            .values()
            .filter(|event| {
                event.date.week(Weekday::Mon).first_day()
                    == required_date.week(Weekday::Mon).first_day()
            })
            .cloned()
            .collect();

        info!("Ивенты с датой: {} получены из памяти с учетом недели", required_date.to_string());

        Ok(EventResult::new(events))
    }

    // получение ивентов в месяце по дате
    pub async fn events_for_month(
        &self,
        required_date: NaiveDate,
    ) -> Result<EventResult<Vec<Event>>, ServerError> {
        let event_map = self.event_map.lock().await;

        // получение ивентов в месяце по дате
        let events: Vec<Event> = event_map
            .values()
            .filter(|event| {
                event.date.year() == required_date.year()
                    && event.date.month() == required_date.month()
            })
            .cloned()
            .collect();

        info!("Ивенты с датой: {} получены из памяти с учетом месяца", required_date.to_string());

        Ok(EventResult::new(events))
    }
}
