use chrono::NaiveDateTime;
use tauri::State;

use crate::{
    db::{
        concerts_controller,
        db_connection_pool::DbConnectionPool,
        transaction_storage::{TransactionId, TransactionStorage},
    },
    model::{concert::Concert, date_time_custom_serde::DATE_TIME_FORMAT},
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_concert(
    date: String,
    duration_minutes: i32,
    address: String,
    name: String,
) -> Result<Concert, String> {
    let date = NaiveDateTime::parse_from_str(&date, DATE_TIME_FORMAT)
        .map_err(|e| format!("Error parsing date: {:?}", e))?;
    Ok(Concert::new(date, duration_minutes, address, name))
}

#[tauri::command]
pub async fn get_all_concerts<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<Concert>, String> {
    let pool = &*connection.connection.lock().await;
    let concerts = concerts_controller::get_all_concerts(pool)
        .await
        .map_err(db_error)?;
    Ok(concerts)
}

#[tauri::command]
pub async fn get_concerts_count<'r>(connection: State<'r, DbConnectionPool>) -> Result<i64, String> {
    let pool = &*connection.connection.lock().await;
    let concerts_count = concerts_controller::get_concerts_count(pool)
        .await
        .map_err(db_error)?;
    Ok(concerts_count)
}

#[tauri::command]
pub async fn get_concerts_paginated<'r>(
    count: i64,
    offset: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<Concert>, String> {
    let pool = &*connection.connection.lock().await;
    let concerts = concerts_controller::get_concerts_paginated(pool, count, offset)
        .await
        .map_err(db_error)?;
    Ok(concerts)
}

#[tauri::command]
pub async fn get_all_concert_ids<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<i64>, String> {
    let pool = &*connection.connection.lock().await;
    let concert_ids = concerts_controller::get_all_concert_ids(pool)
        .await
        .map_err(db_error)?;
    Ok(concert_ids)
}

#[tauri::command]
pub async fn get_all_concert_ids_and_names<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<(i64, String)>, String> {
    let pool = &*connection.connection.lock().await;
    let concert_names = concerts_controller::get_all_concert_ids_and_names(pool)
        .await
        .map_err(db_error)?;
    Ok(concert_names)
}

#[tauri::command]
pub async fn get_concert_by_id<'r>(
    connection: State<'r, DbConnectionPool>,
    concert_id: i64,
) -> Result<Option<Concert>, String> {
    let pool = &*connection.connection.lock().await;
    let concert = concerts_controller::get_concert_by_id(pool, concert_id)
        .await
        .map_err(db_error)?;
    Ok(concert)
}

#[tauri::command]
pub async fn add_concert<'r>(
    concert: Concert,
    connection: State<'r, DbConnectionPool>,
) -> Result<i64, String> {
    let pool = &*connection.connection.lock().await;
    let concert_id = concerts_controller::add_concert(pool, &concert)
        .await
        .map_err(db_error)?;
    Ok(concert_id)
}

#[tauri::command]
pub async fn add_concert_transaction<'r, 't>(
    concert: Concert,
    connection: State<'r, DbConnectionPool>,
    transaction_storage: State<'r, TransactionStorage<'t>>,
) -> Result<(TransactionId, i64), String> {
    let pool = &*connection.connection.lock().await;
    let transaction_storage = &mut *transaction_storage.transactions.lock().await;

    let (transaction, concert_id) = concerts_controller::add_concert_transaction(&pool, &concert)
        .await
        .map_err(db_error)?;

    let transaction_id = TransactionId::Concert(concert_id);
    transaction_storage.insert(transaction_id, transaction);
    Ok((transaction_id, concert_id))
}

#[tauri::command]
pub async fn update_concert<'r>(
    concert: Concert,
    connection: State<'r, DbConnectionPool>,
) -> Result<(), String> {
    let pool = &*connection.connection.lock().await;
    let concert_id = concerts_controller::update_concert(pool, &concert)
        .await
        .map_err(db_error)?;
    Ok(concert_id)
}

#[tauri::command]
pub async fn remove_concert<'r>(
    concert_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<u64, String> {
    let pool = &*connection.connection.lock().await;
    let rows_affected = concerts_controller::remove_concert(pool, concert_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}
