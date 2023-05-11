use tauri::State;

use crate::{
    model::{
        group_artist::{GroupArtist, GroupArtistsRepository},
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_group_artist(
    group_id: i64,
    artist_id: i64,
    role: String,
) -> Result<GroupArtist, String> {
    Ok(GroupArtist::new(group_id, artist_id, role))
}

#[tauri::command]
pub async fn get_all_group_artists<'r>(
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<Vec<GroupArtist>, String> {
    let group_artists = group_artists_repository.get_all().await.map_err(db_error)?;
    Ok(group_artists)
}

#[tauri::command]
pub async fn get_group_artists_count<'r>(
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<u64, String> {
    let group_artists_count = group_artists_repository
        .get_count()
        .await
        .map_err(db_error)?;
    Ok(group_artists_count)
}

#[tauri::command]
pub async fn get_group_artists_paginated<'r>(
    count: i64,
    offset: i64,
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<Vec<GroupArtist>, String> {
    let group_artists = group_artists_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(group_artists)
}

#[tauri::command]
pub async fn get_all_group_artist_ids<'r>(
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<Vec<i64>, String> {
    let group_artist_ids = group_artists_repository.get_ids().await.map_err(db_error)?;
    Ok(group_artist_ids)
}

#[tauri::command]
pub async fn get_group_artist_by_id<'r>(
    group_artist_id: u64,
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<Option<GroupArtist>, String> {
    let group_artist = group_artists_repository
        .get_by_id(group_artist_id)
        .await
        .map_err(db_error)?;
    Ok(group_artist)
}

#[tauri::command]
pub async fn add_group_artist<'r>(
    group_artist: GroupArtist,
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<u64, String> {
    let group_artist_id = group_artists_repository
        .add(&group_artist)
        .await
        .map_err(db_error)?;
    Ok(group_artist_id)
}

#[tauri::command]
pub async fn update_group_artist<'r>(
    group_artist: GroupArtist,
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<(), String> {
    group_artists_repository
        .update(&group_artist)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_group_artist<'r>(
    group_artist_id: u64,
    group_artists_repository: State<'r, GroupArtistsRepository>,
) -> Result<u64, String> {
    let rows_affected = group_artists_repository
        .remove(group_artist_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}
