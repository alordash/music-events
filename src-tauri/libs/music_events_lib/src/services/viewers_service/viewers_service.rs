use tauri::State;

use crate::{
    model::{
        repository::*,
        viewer::{Viewer, ViewersRepository},
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_viewer(person_id: i64, viewer_seat_id: i64) -> Result<Viewer, String> {
    Ok(Viewer::new(person_id, viewer_seat_id))
}

#[tauri::command]
pub async fn get_all_viewers<'r>(
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<Vec<Viewer>, String> {
    let viewers = viewers_repository.get_all().await.map_err(db_error)?;
    Ok(viewers)
}

#[tauri::command]
pub async fn get_viewers_count<'r>(
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<u64, String> {
    let viewers_count = viewers_repository.get_count().await.map_err(db_error)?;
    Ok(viewers_count)
}

#[tauri::command]
pub async fn get_viewers_paginated<'r>(
    count: i64,
    offset: i64,
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<Vec<Viewer>, String> {
    let viewers = viewers_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(viewers)
}

#[tauri::command]
pub async fn get_all_viewer_ids<'r>(
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<Vec<i64>, String> {
    let viewer_ids = viewers_repository.get_ids().await.map_err(db_error)?;
    Ok(viewer_ids)
}

#[tauri::command]
pub async fn get_viewer_by_id<'r>(
    viewer_id: u64,
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<Option<Viewer>, String> {
    let viewer = viewers_repository
        .get_by_id(viewer_id)
        .await
        .map_err(db_error)?;
    Ok(viewer)
}

#[tauri::command]
pub async fn add_viewer<'r>(
    viewer: Viewer,
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<u64, String> {
    let viewer_id = viewers_repository.add(&viewer).await.map_err(db_error)?;
    Ok(viewer_id)
}

#[tauri::command]
pub async fn update_viewer<'r>(
    viewer: Viewer,
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<(), String> {
    viewers_repository.update(&viewer).await.map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_viewer<'r>(
    viewer_id: u64,
    viewers_repository: State<'r, ViewersRepository>,
) -> Result<u64, String> {
    let rows_affected = viewers_repository
        .remove(viewer_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}
