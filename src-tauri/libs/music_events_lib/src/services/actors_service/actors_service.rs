use tauri::State;

use crate::{
    model::{
        actor::{Actor, ActorsRepository},
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_actor(pseudonym: String, person_id: i64) -> Result<Actor, String> {
    Ok(Actor::new(pseudonym, person_id))
}

#[tauri::command]
pub async fn get_all_actors<'r>(
    actors_repository: State<'r, ActorsRepository>,
) -> Result<Vec<Actor>, String> {
    let actors = actors_repository.get_all().await.map_err(db_error)?;
    Ok(actors)
}

#[tauri::command]
pub async fn get_actors_count<'r>(
    actors_repository: State<'r, ActorsRepository>,
) -> Result<u64, String> {
    let actors_count = actors_repository.get_count().await.map_err(db_error)?;
    Ok(actors_count)
}

#[tauri::command]
pub async fn get_actors_paginated<'r>(
    count: i64,
    offset: i64,
    actors_repository: State<'r, ActorsRepository>,
) -> Result<Vec<Actor>, String> {
    let actors = actors_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(actors)
}

#[tauri::command]
pub async fn get_all_actor_ids<'r>(
    actors_repository: State<'r, ActorsRepository>,
) -> Result<Vec<i64>, String> {
    let actor_ids = actors_repository.get_ids().await.map_err(db_error)?;
    Ok(actor_ids)
}

#[tauri::command]
pub async fn get_actor_by_id<'r>(
    actor_id: u64,
    actors_repository: State<'r, ActorsRepository>,
) -> Result<Option<Actor>, String> {
    let actor = actors_repository
        .get_by_id(actor_id)
        .await
        .map_err(db_error)?;
    Ok(actor)
}

#[tauri::command]
pub async fn add_actor<'r>(
    actor: Actor,
    actors_repository: State<'r, ActorsRepository>,
) -> Result<u64, String> {
    let actor_id = actors_repository.add(&actor).await.map_err(db_error)?;
    Ok(actor_id)
}

#[tauri::command]
pub async fn update_actor<'r>(
    actor: Actor,
    actors_repository: State<'r, ActorsRepository>,
) -> Result<(), String> {
    actors_repository.update(&actor).await.map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_actor<'r>(
    actor_id: u64,
    actors_repository: State<'r, ActorsRepository>,
) -> Result<u64, String> {
    let rows_affected = actors_repository.remove(actor_id).await.map_err(db_error)?;
    Ok(rows_affected)
}
