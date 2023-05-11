use tauri::State;

use crate::{
    model::{
        person::{Person, PersonsRepository},
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_person(name: String, surname: String) -> Result<Person, String> {
    Ok(Person::new(name, surname))
}

#[tauri::command]
pub async fn get_all_persons<'r>(
    persons_repository: State<'r, PersonsRepository>,
) -> Result<Vec<Person>, String> {
    let persons = persons_repository.get_all().await.map_err(db_error)?;
    Ok(persons)
}

#[tauri::command]
pub async fn get_persons_count<'r>(
    persons_repository: State<'r, PersonsRepository>,
) -> Result<u64, String> {
    let persons_count = persons_repository.get_count().await.map_err(db_error)?;
    Ok(persons_count)
}

#[tauri::command]
pub async fn get_persons_paginated<'r>(
    count: i64,
    offset: i64,
    persons_repository: State<'r, PersonsRepository>,
) -> Result<Vec<Person>, String> {
    let persons = persons_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(persons)
}

#[tauri::command]
pub async fn get_all_person_ids<'r>(
    persons_repository: State<'r, PersonsRepository>,
) -> Result<Vec<i64>, String> {
    let person_ids = persons_repository.get_ids().await.map_err(db_error)?;
    Ok(person_ids)
}

#[tauri::command]
pub async fn get_person_by_id<'r>(
    person_id: u64,
    persons_repository: State<'r, PersonsRepository>,
) -> Result<Option<Person>, String> {
    let person = persons_repository
        .get_by_id(person_id)
        .await
        .map_err(db_error)?;
    Ok(person)
}

#[tauri::command]
pub async fn add_person<'r>(
    person: Person,
    persons_repository: State<'r, PersonsRepository>,
) -> Result<u64, String> {
    let person_id = persons_repository.add(&person).await.map_err(db_error)?;
    Ok(person_id)
}

#[tauri::command]
pub async fn update_person<'r>(
    person: Person,
    persons_repository: State<'r, PersonsRepository>,
) -> Result<(), String> {
    persons_repository.update(&person).await.map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_person<'r>(
    person_id: u64,
    persons_repository: State<'r, PersonsRepository>,
) -> Result<u64, String> {
    let rows_affected = persons_repository
        .remove(person_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}
