use crate::model::{Event, EventModel, ServerError};
use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;
use chrono::NaiveDate;

// POST /create_event - добавление ивента в память
pub async fn create_event(
    State(event_model): State<Arc<EventModel>>,
    Json(order): Json<Event>,
) -> Result<(), ServerError> {
    // добавление ивента в память
    event_model.create_event(order).await?;

    Ok(())
}

// POST /update_event - обновить данные об ивенте в памяти
pub async fn update_event(
    State(event_model): State<Arc<EventModel>>,
    Json(order): Json<Event>,
) -> Result<(), ServerError> {
    // обновить данные об ивенте
    event_model.update_event(order).await?;

    Ok(())
}

// POST /delete_event - удалить данные об ивенте в памяти
pub async fn delete_event(
    State(event_model): State<Arc<EventModel>>,
    Path(event_id): Path<u32>,
) -> Result<(), ServerError> {
    // удалить данные об ивенте
    event_model.delete_event(event_id).await?;

    Ok(())
}

pub async fn events_for_day(
    State(event_map_mutex): State<Arc<EventModel>>,
    Path(date): Path<NaiveDate>,
) -> Result<Json<Vec<Event>>, ServerError> {
    // получить данные об иветах в дне по дате
    let events_for_day = event_map_mutex.events_for_day(date).await?;

    Ok(events_for_day)
}

pub async fn events_for_week(
    State(event_map_mutex): State<Arc<EventModel>>,
    Path(date): Path<NaiveDate>,
) -> Result<Json<Vec<Event>>, ServerError> {
    // получить данные об ивентах на неделе по дате
    let events_for_week = event_map_mutex.events_for_week(date).await?;

    Ok(events_for_week)
}

pub async fn events_for_month(
    State(event_map_mutex): State<Arc<EventModel>>,
    Path(date): Path<NaiveDate>,
) -> Result<Json<Vec<Event>>, ServerError> {
    // получить данные об ивентах в месяце по дате
    let events_for_month = event_map_mutex.events_for_month(date).await?;

    Ok(events_for_month)
}
