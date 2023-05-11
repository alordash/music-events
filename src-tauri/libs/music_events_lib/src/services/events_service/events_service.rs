use chrono::NaiveDateTime;
use tauri::State;

use crate::{
    model::{
        event::{Event, EventsRepository},
        date_time_custom_serde::DATE_TIME_FORMAT,
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_event(
    name: String,
) -> Result<Event, String> {
    Ok(Event::new(
        name
    ))
}

#[tauri::command]
pub async fn get_all_events<'r>(
    events_repository: State<'r, EventsRepository>,
) -> Result<Vec<Event>, String> {
    let events = events_repository.get_all().await.map_err(db_error)?;
    Ok(events)
}

#[tauri::command]
pub async fn get_events_count<'r>(
    events_repository: State<'r, EventsRepository>,
) -> Result<u64, String> {
    let events_count = events_repository.get_count().await.map_err(db_error)?;
    Ok(events_count)
}

#[tauri::command]
pub async fn get_events_paginated<'r>(
    count: i64,
    offset: i64,
    events_repository: State<'r, EventsRepository>,
) -> Result<Vec<Event>, String> {
    let events = events_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(events)
}

#[tauri::command]
pub async fn get_all_event_ids<'r>(
    events_repository: State<'r, EventsRepository>,
) -> Result<Vec<i64>, String> {
    let event_ids = events_repository.get_ids().await.map_err(db_error)?;
    Ok(event_ids)
}

#[tauri::command]
pub async fn get_event_by_id<'r>(
    event_id: u64,
    events_repository: State<'r, EventsRepository>,
) -> Result<Option<Event>, String> {
    let event = events_repository
        .get_by_id(event_id)
        .await
        .map_err(db_error)?;
    Ok(event)
}

#[tauri::command]
pub async fn add_event<'r>(
    event: Event,
    events_repository: State<'r, EventsRepository>,
) -> Result<u64, String> {
    let event_id = events_repository.add(&event).await.map_err(db_error)?;
    Ok(event_id)
}

#[tauri::command]
pub async fn update_event<'r>(
    event: Event,
    events_repository: State<'r, EventsRepository>,
) -> Result<(), String> {
    events_repository
        .update(&event)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_event<'r>(
    event_id: u64,
    events_repository: State<'r, EventsRepository>,
) -> Result<u64, String> {
    let rows_affected = events_repository
        .remove(event_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}
