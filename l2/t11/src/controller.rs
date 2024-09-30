use crate::model::{Event, EventModel, EventResult, ServerError};
use axum::extract::{Path, State};
use axum::Json;
use chrono::NaiveDate;
use std::sync::Arc;

// POST /create_event - добавление ивента в память
pub async fn create_event(
    State(event_model): State<Arc<EventModel>>,
    Json(event): Json<Event>,
) -> Result<Json<EventResult<Event>>, ServerError> {
    // добавление ивента в память
    let model_result = event_model.create_event(event).await?;

    Ok(Json(model_result))
}

// POST /update_event - обновить данные об ивенте в памяти
pub async fn update_event(
    State(event_model): State<Arc<EventModel>>,
    Json(order): Json<Event>,
) -> Result<Json<EventResult<Event>>, ServerError> {
    // обновить данные об ивенте
    let model_result = event_model.update_event(order).await?;

    Ok(Json(model_result))
}

// POST /delete_event - удалить данные об ивенте в памяти
pub async fn delete_event(
    State(event_model): State<Arc<EventModel>>,
    Path(event_id): Path<u32>,
) -> Result<Json<EventResult<Event>>, ServerError> {
    // удалить данные об ивенте
    let model_result = event_model.delete_event(event_id).await?;

    Ok(Json(model_result))
}

// GET /events_for_day - получить данные об ивенте в дне по дате из памяти
pub async fn events_for_day(
    State(event_map_mutex): State<Arc<EventModel>>,
    Path(date): Path<NaiveDate>,
) -> Result<Json<EventResult<Vec<Event>>>, ServerError> {
    // получить данные об иветах в дне по дате
    let model_result = event_map_mutex.events_for_day(date).await?;

    Ok(Json(model_result))
}

// GET /events_for_week - получить данные об ивенте в неделе по дате из памяти
pub async fn events_for_week(
    State(event_map_mutex): State<Arc<EventModel>>,
    Path(date): Path<NaiveDate>,
) -> Result<Json<EventResult<Vec<Event>>>, ServerError> {
    // получить данные об ивентах на неделе по дате
    let model_result = event_map_mutex.events_for_week(date).await?;

    Ok(Json(model_result))
}

// GET /events_for_month - получить данные об ивенте в месяце по дате из памяти
pub async fn events_for_month(
    State(event_map_mutex): State<Arc<EventModel>>,
    Path(date): Path<NaiveDate>,
) -> Result<Json<EventResult<Vec<Event>>>, ServerError> {
    // получить данные об ивентах в месяце по дате
    let model_result = event_map_mutex.events_for_month(date).await?;

    Ok(Json(model_result))
}
