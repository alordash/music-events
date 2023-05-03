use sqlx::types::Decimal;
use tauri::State;

use crate::{
    db::{
        db_connection_pool::DbConnectionPool,
        transaction_storage::{TransactionId, TransactionStorage},
        viewer_seats_controller,
    },
    model::viewer_seat::ViewerSeat,
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_viewer_seat(
    kind: String,
    cost_rubles: Decimal,
    real_number: i32,
    concert_id: i64,
) -> ViewerSeat {
    ViewerSeat::new(kind, cost_rubles, real_number, concert_id)
}

#[tauri::command]
pub async fn get_viewer_seats_paginated<'r>(
    count: i64,
    offset: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<ViewerSeat>, String> {
    let pool = &*connection.connection.lock().await;
    let viewer_seats = viewer_seats_controller::get_viewer_seats_paginated(pool, count, offset)
        .await
        .map_err(db_error)?;
    Ok(viewer_seats)
}

#[tauri::command]
pub async fn get_concert_viewer_seats<'r>(
    concert_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<ViewerSeat>, String> {
    let pool = &*connection.connection.lock().await;
    let viewer_seats = viewer_seats_controller::get_concert_viewer_seats(pool, concert_id)
        .await
        .map_err(db_error)?;
    Ok(viewer_seats)
}

#[tauri::command]
pub async fn get_all_viewer_seats<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<ViewerSeat>, String> {
    let pool = &*connection.connection.lock().await;
    let viewer_seats = viewer_seats_controller::get_all_viewer_seats(pool)
        .await
        .map_err(db_error)?;
    Ok(viewer_seats)
}

#[tauri::command]
pub async fn get_all_viewer_seat_ids_and_real_numbers_and_concert_names<'r>(
    connection: State<'r, DbConnectionPool>,
) -> Result<Vec<(i64, i32, String)>, String> {
    let pool = &*connection.connection.lock().await;
    let viewer_seats_ids_and_real_numbers =
        viewer_seats_controller::get_all_viewer_seat_ids_and_real_numbers_and_concert_names(pool)
            .await
            .map_err(db_error)?;
    Ok(viewer_seats_ids_and_real_numbers)
}

#[tauri::command]
pub async fn get_viewer_seat_by_id<'r>(
    connection: State<'r, DbConnectionPool>,
    viewer_seat_id: i64,
) -> Result<Option<ViewerSeat>, String> {
    let pool = &*connection.connection.lock().await;
    let viewer_seat = viewer_seats_controller::get_viewer_seat_by_id(pool, viewer_seat_id)
        .await
        .map_err(db_error)?;
    Ok(viewer_seat)
}

#[tauri::command]
pub async fn add_viewer_seat<'r>(
    viewer_seat: ViewerSeat,
    connection: State<'r, DbConnectionPool>,
) -> Result<i64, String> {
    let pool = &*connection.connection.lock().await;

    let viewer_seat_id = viewer_seats_controller::add_viewer_seat(pool, &viewer_seat)
        .await
        .map_err(db_error)?;
    Ok(viewer_seat_id)
}

#[tauri::command]
pub async fn update_viewer_seat<'r>(
    viewer_seat: ViewerSeat,
    connection: State<'r, DbConnectionPool>,
) -> Result<(), String> {
    let pool = &*connection.connection.lock().await;

    viewer_seats_controller::update_viewer_seat(pool, &viewer_seat)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn update_viewer_seat_transaction<'r, 't>(
    viewer_seat: ViewerSeat,
    connection: State<'r, DbConnectionPool>,
    transaction_storage: State<'r, TransactionStorage<'t>>,
) -> Result<TransactionId, String> {
    let pool = &*connection.connection.lock().await;
    let transaction_storage = &mut *transaction_storage.transactions.lock().await;

    let transaction = viewer_seats_controller::update_viewer_seat_transaction(&pool, &viewer_seat)
        .await
        .map_err(db_error)?;

    let transaction_id = TransactionId::ViewerSeat(viewer_seat.id().unwrap());
    transaction_storage.insert(transaction_id, transaction);
    Ok(transaction_id)
}

#[tauri::command]
pub async fn remove_viewer_seat<'r>(
    viewer_seat_id: i64,
    connection: State<'r, DbConnectionPool>,
) -> Result<u64, String> {
    let pool = &*connection.connection.lock().await;
    let rows_affected = viewer_seats_controller::remove_viewer_seat(pool, viewer_seat_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn remove_viewer_seat_transaction<'r, 't>(
    viewer_seat_id: i64,
    connection: State<'r, DbConnectionPool>,
    transaction_storage: State<'r, TransactionStorage<'t>>,
) -> Result<TransactionId, String> {
    let pool = &*connection.connection.lock().await;
    let transaction_storage = &mut *transaction_storage.transactions.lock().await;

    let transaction =
        viewer_seats_controller::remove_viewer_seat_transaction(&pool, viewer_seat_id)
            .await
            .map_err(db_error)?;

    let transaction_id = TransactionId::ViewerSeat(viewer_seat_id);
    transaction_storage.insert(transaction_id, transaction);

    Ok(transaction_id)
}
