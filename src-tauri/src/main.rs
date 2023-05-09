#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(async_fn_in_trait)]

use std::sync::Arc;

use music_events_lib::db::db_connection_pool::establish_connection_pool;
use music_events_lib::db::transaction_storage::TransactionStorage;
use music_events_lib::model::concert::ConcertsRepository;
use music_events_lib::model::viewer_seat::viewer_seats_repository::ViewerSeatsRepository;
use music_events_lib::services::concerts_service::concerts_service::*;
use music_events_lib::services::general_service::general_service::*;
use music_events_lib::services::viewer_seats_service::viewer_seats_service::*;
use tauri::Manager;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = Arc::new(establish_connection_pool().await.unwrap());
    let concerts_repository = ConcertsRepository::new(pool.clone());
    let viewer_seats_repository = ViewerSeatsRepository::new(pool.clone());

    let transaction_storage = TransactionStorage::new();
    tauri::Builder::default()
        .manage(concerts_repository)
        .manage(viewer_seats_repository)
        .manage(transaction_storage)
        .invoke_handler(tauri::generate_handler![
            create_concert,
            create_viewer_seat,
            get_all_concerts,
            get_concerts_count,
            get_concerts_paginated,
            get_all_concert_ids,
            get_concert_by_id,
            get_concert_viewer_seats,
            add_concert,
            update_concert,
            remove_concert,
            get_viewer_seats_paginated,
            get_all_viewer_seats,
            // get_all_viewer_seat_ids_and_real_numbers_and_concert_names,
            get_viewer_seat_by_id,
            add_viewer_seat,
            update_viewer_seat,
            // update_viewer_seat_transaction,
            remove_viewer_seat,
            // remove_viewer_seat_transaction,
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
