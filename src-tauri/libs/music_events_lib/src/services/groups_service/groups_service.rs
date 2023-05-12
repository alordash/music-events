use tauri::State;

use crate::{
    model::{
        group::{Group, GroupsRepository},
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_group(name: String, genre: String) -> Result<Group, String> {
    Ok(Group::new(name, genre))
}

#[tauri::command]
pub async fn get_all_groups<'r>(
    groups_repository: State<'r, GroupsRepository>,
) -> Result<Vec<Group>, String> {
    let groups = groups_repository.get_all().await.map_err(db_error)?;
    Ok(groups)
}

#[tauri::command]
pub async fn get_groups_count<'r>(
    groups_repository: State<'r, GroupsRepository>,
) -> Result<u64, String> {
    let groups_count = groups_repository.get_count().await.map_err(db_error)?;
    Ok(groups_count)
}

#[tauri::command]
pub async fn get_groups_paginated<'r>(
    count: i64,
    offset: i64,
    groups_repository: State<'r, GroupsRepository>,
) -> Result<Vec<Group>, String> {
    let groups = groups_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(groups)
}

#[tauri::command]
pub async fn get_all_group_ids<'r>(
    groups_repository: State<'r, GroupsRepository>,
) -> Result<Vec<i64>, String> {
    let group_ids = groups_repository.get_ids().await.map_err(db_error)?;
    Ok(group_ids)
}

#[tauri::command]
pub async fn get_group_by_id<'r>(
    group_id: u64,
    groups_repository: State<'r, GroupsRepository>,
) -> Result<Option<Group>, String> {
    let group = groups_repository
        .get_by_id(group_id)
        .await
        .map_err(db_error)?;
    Ok(group)
}

#[tauri::command]
pub async fn add_group<'r>(
    group: Group,
    groups_repository: State<'r, GroupsRepository>,
) -> Result<u64, String> {
    let group_id = groups_repository.add(&group).await.map_err(db_error)?;
    Ok(group_id)
}

#[tauri::command]
pub async fn update_group<'r>(
    group: Group,
    groups_repository: State<'r, GroupsRepository>,
) -> Result<(), String> {
    groups_repository.update(&group).await.map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_group<'r>(
    group_id: u64,
    groups_repository: State<'r, GroupsRepository>,
) -> Result<u64, String> {
    let rows_affected = groups_repository.remove(group_id).await.map_err(db_error)?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn get_concert_groups<'r>(
    concert_id: u64,
    groups_repository: State<'r, GroupsRepository>,
) -> Result<Vec<Group>, String> {
    let groups = groups_repository
        .get_concert_groups(concert_id)
        .await
        .map_err(db_error)?;
    Ok(groups)
}
