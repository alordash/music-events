#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod db;
pub mod model;
pub mod services;

use db::db_connection_pool::establish_connection_pool;
use db::transaction_storage::TransactionStorage;
use services::concerts_service::concerts_service::*;
use services::general_service::general_service::*;
use services::viewer_seats_service::viewer_seats_service::*;
use tauri::Manager;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let connections = establish_connection_pool().await.unwrap();
    let transaction_storage = TransactionStorage::new();
    tauri::Builder::default()
        .manage(connections) // Makes connection pool available in all #[tauri::command]
        .manage(transaction_storage)
        .invoke_handler(tauri::generate_handler![
            create_concert,
            create_viewer_seat,
            get_all_concerts,
            get_concerts_count,
            get_concerts_paginated,
            get_all_concert_ids,
            get_all_concert_ids_and_names,
            get_concert_by_id,
            get_concert_viewer_seats,
            add_concert,
            add_concert_transaction,
            update_concert,
            remove_concert,
            get_viewer_seats_paginated,
            get_all_viewer_seats,
            get_all_viewer_seat_ids_and_real_numbers_and_concert_names,
            get_viewer_seat_by_id,
            add_viewer_seat,
            update_viewer_seat,
            update_viewer_seat_transaction,
            remove_viewer_seat,
            remove_viewer_seat_transaction,
            transaction_commit,
            transaction_rollback
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
