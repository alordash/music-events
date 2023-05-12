use tauri::State;

use crate::{
    model::{
        person::Person,
        repository::*,
        user::{User, UsersRepository},
        viewer::Viewer,
        viewer_seat::ViewerSeat,
    },
    services::db_error::db_error,
};

#[tauri::command]
pub fn create_user(login: String, password: String, role: String) -> Result<User, String> {
    Ok(User::new(login, password, role))
}

#[tauri::command]
pub async fn get_all_users<'r>(
    users_repository: State<'r, UsersRepository>,
) -> Result<Vec<User>, String> {
    let users = users_repository.get_all().await.map_err(db_error)?;
    Ok(users)
}

#[tauri::command]
pub async fn get_users_count<'r>(
    users_repository: State<'r, UsersRepository>,
) -> Result<u64, String> {
    let users_count = users_repository.get_count().await.map_err(db_error)?;
    Ok(users_count)
}

#[tauri::command]
pub async fn get_users_paginated<'r>(
    count: i64,
    offset: i64,
    users_repository: State<'r, UsersRepository>,
) -> Result<Vec<User>, String> {
    let users = users_repository
        .get_paginated(count, offset)
        .await
        .map_err(db_error)?;
    Ok(users)
}

#[tauri::command]
pub async fn get_all_user_ids<'r>(
    users_repository: State<'r, UsersRepository>,
) -> Result<Vec<i64>, String> {
    let user_ids = users_repository.get_ids().await.map_err(db_error)?;
    Ok(user_ids)
}

#[tauri::command]
pub async fn get_user_by_id<'r>(
    user_id: u64,
    users_repository: State<'r, UsersRepository>,
) -> Result<Option<User>, String> {
    let user = users_repository
        .get_by_id(user_id)
        .await
        .map_err(db_error)?;
    Ok(user)
}

#[tauri::command]
pub async fn add_user<'r>(
    user: User,
    users_repository: State<'r, UsersRepository>,
) -> Result<u64, String> {
    let user_id = users_repository.add(&user).await.map_err(db_error)?;
    Ok(user_id)
}

#[tauri::command]
pub async fn update_user<'r>(
    user: User,
    users_repository: State<'r, UsersRepository>,
) -> Result<(), String> {
    users_repository.update(&user).await.map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_user<'r>(
    user_id: u64,
    users_repository: State<'r, UsersRepository>,
) -> Result<u64, String> {
    let rows_affected = users_repository.remove(user_id).await.map_err(db_error)?;
    Ok(rows_affected)
}

#[tauri::command]
pub async fn try_login_user<'r>(
    login: String,
    password: String,
    users_repository: State<'r, UsersRepository>,
) -> Result<Option<User>, String> {
    let maybe_user = users_repository
        .try_login(login, password)
        .await
        .map_err(db_error)?;
    Ok(maybe_user)
}

#[tauri::command]
pub async fn get_bought_viewer_seats<'r>(
    user_id: u64,
    users_repository: State<'r, UsersRepository>,
) -> Result<Vec<(Person, Viewer, ViewerSeat)>, String> {
    let persons_viewer_seats = users_repository
        .get_bought_viewer_seats(user_id)
        .await
        .map_err(db_error)
        .map_err(|s| format!("BOUGHT VIEWER SEATS ERROR: {}", s))?;
    Ok(persons_viewer_seats)
}
