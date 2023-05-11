use tauri::State;

use crate::{
    model::{
        artist::{Artist, ArtistsRepository},
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_artist(pseudonym: String, person_id: i64) -> Result<Artist, String> {
    Ok(Artist::new(pseudonym, person_id))
}

#[tauri::command]
pub async fn get_all_artists<'r>(
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<Vec<Artist>, String> {
    let artists = artists_repository.get_all().await.map_err(db_error)?;
    Ok(artists)
}

#[tauri::command]
pub async fn get_artists_count<'r>(
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<u64, String> {
    let artists_count = artists_repository.get_count().await.map_err(db_error)?;
    Ok(artists_count)
}

#[tauri::command]
pub async fn get_artists_paginated<'r>(
    count: i64,
    offset: i64,
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<Vec<Artist>, String> {
    let artists = artists_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(artists)
}

#[tauri::command]
pub async fn get_all_artist_ids<'r>(
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<Vec<i64>, String> {
    let artist_ids = artists_repository.get_ids().await.map_err(db_error)?;
    Ok(artist_ids)
}

#[tauri::command]
pub async fn get_artist_by_id<'r>(
    artist_id: u64,
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<Option<Artist>, String> {
    let artist = artists_repository
        .get_by_id(artist_id)
        .await
        .map_err(db_error)?;
    Ok(artist)
}

#[tauri::command]
pub async fn add_artist<'r>(
    artist: Artist,
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<u64, String> {
    let artist_id = artists_repository.add(&artist).await.map_err(db_error)?;
    Ok(artist_id)
}

#[tauri::command]
pub async fn update_artist<'r>(
    artist: Artist,
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<(), String> {
    artists_repository.update(&artist).await.map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_artist<'r>(
    artist_id: u64,
    artists_repository: State<'r, ArtistsRepository>,
) -> Result<u64, String> {
    let rows_affected = artists_repository.remove(artist_id).await.map_err(db_error)?;
    Ok(rows_affected)
}
