use sqlx::types::Decimal;
use tauri::State;

use crate::{
    db::{
        db_connection_pool::DbConnectionPool,
        transaction_storage::{TransactionId, TransactionStorage},
    },
    model::{
        repository::*,
        viewer_seat::{
            viewer_seats_repository::{self, ViewerSeatsRepository},
            ViewerSeat,
        },
    },
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
    viewer_seats_repository: State<'r, ViewerSeatsRepository>,
) -> Result<Vec<ViewerSeat>, String> {
    let viewer_seats = viewer_seats_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(viewer_seats)
}

#[tauri::command]
pub async fn get_concert_viewer_seats<'r>(
    concert_id: u64,
    viewer_seats_repository: State<'r, ViewerSeatsRepository>,
) -> Result<Vec<ViewerSeat>, String> {
    let viewer_seats = viewer_seats_repository
        .get_concert_viewer_seats(concert_id)
        .await
        .map_err(db_error)?;
    Ok(viewer_seats)
}

#[tauri::command]
pub async fn get_all_viewer_seats<'r>(
    viewer_seats_repository: State<'r, ViewerSeatsRepository>,
) -> Result<Vec<ViewerSeat>, String> {
    let viewer_seats = viewer_seats_repository.get_all().await.map_err(db_error)?;
    Ok(viewer_seats)
}

#[tauri::command]
pub async fn get_viewer_seat_by_id<'r>(
    viewer_seat_id: u64,
    viewer_seats_repository: State<'r, ViewerSeatsRepository>,
) -> Result<Option<ViewerSeat>, String> {
    let viewer_seat = viewer_seats_repository
        .get_by_id(viewer_seat_id)
        .await
        .map_err(db_error)?;
    Ok(viewer_seat)
}

#[tauri::command]
pub async fn add_viewer_seat<'r>(
    viewer_seat: ViewerSeat,
    viewer_seats_repository: State<'r, ViewerSeatsRepository>,
) -> Result<u64, String> {
    let viewer_seat_id = viewer_seats_repository
        .add(&viewer_seat)
        .await
        .map_err(db_error)?;
    Ok(viewer_seat_id)
}

#[tauri::command]
pub async fn update_viewer_seat<'r>(
    viewer_seat: ViewerSeat,
    viewer_seats_repository: State<'r, ViewerSeatsRepository>,
) -> Result<(), String> {
    viewer_seats_repository
        .update(&viewer_seat)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_viewer_seat<'r>(
    viewer_seat_id: u64,
    viewer_seats_repository: State<'r, ViewerSeatsRepository>,
) -> Result<u64, String> {
    let rows_affected = viewer_seats_repository
        .remove(viewer_seat_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}