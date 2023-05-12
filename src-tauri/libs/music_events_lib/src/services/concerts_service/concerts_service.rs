use chrono::NaiveDateTime;
use tauri::State;

use crate::{
    model::{
        concert::{Concert, ConcertsRepository},
        date_time_custom_serde::DATE_TIME_FORMAT,
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_concert(
    date: String,
    duration_minutes: i32,
    address: String,
    name: String,
    event_id: i64,
) -> Result<Concert, String> {
    let date = NaiveDateTime::parse_from_str(&date, DATE_TIME_FORMAT)
        .map_err(|e| format!("Error parsing date: {:?}", e))?;
    Ok(Concert::new(
        date,
        duration_minutes,
        address,
        name,
        event_id,
    ))
}

#[tauri::command]
pub async fn get_all_concerts<'r>(
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<Vec<Concert>, String> {
    let concerts = concerts_repository.get_all().await.map_err(db_error)?;
    Ok(concerts)
}

#[tauri::command]
pub async fn get_concerts_count<'r>(
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<u64, String> {
    let concerts_count = concerts_repository.get_count().await.map_err(db_error)?;
    Ok(concerts_count)
}

#[tauri::command]
pub async fn get_concerts_paginated<'r>(
    count: i64,
    offset: i64,
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<Vec<Concert>, String> {
    let concerts = concerts_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(concerts)
}

#[tauri::command]
pub async fn get_all_concert_ids<'r>(
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<Vec<i64>, String> {
    let concert_ids = concerts_repository.get_ids().await.map_err(db_error)?;
    Ok(concert_ids)
}

#[tauri::command]
pub async fn get_concert_by_id<'r>(
    concert_id: u64,
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<Option<Concert>, String> {
    let concert = concerts_repository
        .get_by_id(concert_id)
        .await
        .map_err(db_error)?;
    Ok(concert)
}

#[tauri::command]
pub async fn add_concert<'r>(
    concert: Concert,
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<u64, String> {
    let concert_id = concerts_repository.add(&concert).await.map_err(db_error)?;
    Ok(concert_id)
}

#[tauri::command]
pub async fn update_concert<'r>(
    concert: Concert,
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<(), String> {
    concerts_repository
        .update(&concert)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_concert<'r>(
    concert_id: u64,
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<u64, String> {
    let rows_affected = concerts_repository
        .remove(concert_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn get_event_concerts<'r>(
    event_id: u64,
    concerts_repository: State<'r, ConcertsRepository>,
) -> Result<Vec<Concert>, String> {
    let concerts = concerts_repository
        .get_event_concerts(event_id)
        .await
        .map_err(db_error)?;
    Ok(concerts)
}
