use tauri::State;

use crate::{
    model::{
        repository::*,
        user_person::{UserPerson, UserPersonsRepository},
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_user_person(user_id: i64, person_id: i64) -> Result<UserPerson, String> {
    Ok(UserPerson::new(user_id, person_id))
}

#[tauri::command]
pub async fn get_all_user_persons<'r>(
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<Vec<UserPerson>, String> {
    let user_persons = user_persons_repository.get_all().await.map_err(db_error)?;
    Ok(user_persons)
}

#[tauri::command]
pub async fn get_user_persons_count<'r>(
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<u64, String> {
    let user_persons_count = user_persons_repository
        .get_count()
        .await
        .map_err(db_error)?;
    Ok(user_persons_count)
}

#[tauri::command]
pub async fn get_user_persons_paginated<'r>(
    count: i64,
    offset: i64,
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<Vec<UserPerson>, String> {
    let user_persons = user_persons_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(user_persons)
}

#[tauri::command]
pub async fn get_all_user_person_ids<'r>(
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<Vec<i64>, String> {
    let user_person_ids = user_persons_repository.get_ids().await.map_err(db_error)?;
    Ok(user_person_ids)
}

#[tauri::command]
pub async fn get_user_person_by_id<'r>(
    user_person_id: u64,
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<Option<UserPerson>, String> {
    let user_person = user_persons_repository
        .get_by_id(user_person_id)
        .await
        .map_err(db_error)?;
    Ok(user_person)
}

#[tauri::command]
pub async fn add_user_person<'r>(
    user_person: UserPerson,
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<u64, String> {
    let user_person_id = user_persons_repository
        .add(&user_person)
        .await
        .map_err(db_error)?;
    Ok(user_person_id)
}

#[tauri::command]
pub async fn update_user_person<'r>(
    user_person: UserPerson,
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<(), String> {
    user_persons_repository
        .update(&user_person)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_user_person<'r>(
    user_person_id: u64,
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<u64, String> {
    let rows_affected = user_persons_repository
        .remove(user_person_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn get_user_person_by_user_id_and_person_id<'r>(
    user_id: u64,
    person_id: u64,
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<Option<UserPerson>, String> {
    let user_person = user_persons_repository
        .get_user_person_by_user_id_and_person_id(user_id, person_id)
        .await
        .map_err(db_error)?;
    Ok(user_person)
}

#[tauri::command]
pub async fn remove_user_person_by_person_id<'r>(
    person_id: u64,
    user_persons_repository: State<'r, UserPersonsRepository>,
) -> Result<u64, String> {
    let rows_affected = user_persons_repository
        .remove_user_person_by_person_id(person_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}
