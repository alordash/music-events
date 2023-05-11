use tauri::State;

use crate::{
    model::{
        repertoire::{Repertoire, RepertoiresRepository},
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_repertoire(
    name: String,
    order_number: i32,
    participant_id: i64,
) -> Result<Repertoire, String> {
    Ok(Repertoire::new(name, order_number, participant_id))
}

#[tauri::command]
pub async fn get_all_repertoires<'r>(
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<Vec<Repertoire>, String> {
    let repertoires = repertoires_repository.get_all().await.map_err(db_error)?;
    Ok(repertoires)
}

#[tauri::command]
pub async fn get_repertoires_count<'r>(
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<u64, String> {
    let repertoires_count = repertoires_repository.get_count().await.map_err(db_error)?;
    Ok(repertoires_count)
}

#[tauri::command]
pub async fn get_repertoires_paginated<'r>(
    count: i64,
    offset: i64,
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<Vec<Repertoire>, String> {
    let repertoires = repertoires_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(repertoires)
}

#[tauri::command]
pub async fn get_all_repertoire_ids<'r>(
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<Vec<i64>, String> {
    let repertoire_ids = repertoires_repository.get_ids().await.map_err(db_error)?;
    Ok(repertoire_ids)
}

#[tauri::command]
pub async fn get_repertoire_by_id<'r>(
    repertoire_id: u64,
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<Option<Repertoire>, String> {
    let repertoire = repertoires_repository
        .get_by_id(repertoire_id)
        .await
        .map_err(db_error)?;
    Ok(repertoire)
}

#[tauri::command]
pub async fn add_repertoire<'r>(
    repertoire: Repertoire,
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<u64, String> {
    let repertoire_id = repertoires_repository
        .add(&repertoire)
        .await
        .map_err(db_error)?;
    Ok(repertoire_id)
}

#[tauri::command]
pub async fn update_repertoire<'r>(
    repertoire: Repertoire,
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<(), String> {
    repertoires_repository
        .update(&repertoire)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_repertoire<'r>(
    repertoire_id: u64,
    repertoires_repository: State<'r, RepertoiresRepository>,
) -> Result<u64, String> {
    let rows_affected = repertoires_repository
        .remove(repertoire_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}
