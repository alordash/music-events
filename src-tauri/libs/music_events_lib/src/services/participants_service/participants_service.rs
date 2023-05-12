use tauri::State;

use crate::{
    model::{
        participant::{Participant, ParticipantsRepository},
        repertoire::Repertoire,
        repository::*,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_participant(concert_id: i64, group_id: i64) -> Result<Participant, String> {
    Ok(Participant::new(concert_id, group_id))
}

#[tauri::command]
pub async fn get_all_participants<'r>(
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<Vec<Participant>, String> {
    let participants = participants_repository.get_all().await.map_err(db_error)?;
    Ok(participants)
}

#[tauri::command]
pub async fn get_participants_count<'r>(
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<u64, String> {
    let participants_count = participants_repository
        .get_count()
        .await
        .map_err(db_error)?;
    Ok(participants_count)
}

#[tauri::command]
pub async fn get_participants_paginated<'r>(
    count: i64,
    offset: i64,
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<Vec<Participant>, String> {
    let participants = participants_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(participants)
}

#[tauri::command]
pub async fn get_all_participant_ids<'r>(
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<Vec<i64>, String> {
    let participant_ids = participants_repository.get_ids().await.map_err(db_error)?;
    Ok(participant_ids)
}

#[tauri::command]
pub async fn get_participant_by_id<'r>(
    participant_id: u64,
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<Option<Participant>, String> {
    let participant = participants_repository
        .get_by_id(participant_id)
        .await
        .map_err(db_error)?;
    Ok(participant)
}

#[tauri::command]
pub async fn add_participant<'r>(
    participant: Participant,
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<u64, String> {
    let participant_id = participants_repository
        .add(&participant)
        .await
        .map_err(db_error)?;
    Ok(participant_id)
}

#[tauri::command]
pub async fn update_participant<'r>(
    participant: Participant,
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<(), String> {
    participants_repository
        .update(&participant)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_participant<'r>(
    participant_id: u64,
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<u64, String> {
    let rows_affected = participants_repository
        .remove(participant_id)
        .await
        .map_err(db_error)?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn get_participant_repertoires<'r>(
    participant_id: u64,
    participants_repository: State<'r, ParticipantsRepository>,
) -> Result<Vec<Repertoire>, String> {
    let participants = participants_repository
        .get_participant_repertoires(participant_id)
        .await
        .map_err(db_error)?;
    Ok(participants)
}
