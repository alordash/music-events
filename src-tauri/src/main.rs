#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use music_events_lib::db::db_connection_pool::establish_connection_pool;
use music_events_lib::db::transaction_storage::TransactionStorage;
use music_events_lib::model::artist::ArtistsRepository;
use music_events_lib::model::concert::ConcertsRepository;
use music_events_lib::model::event::EventsRepository;
use music_events_lib::model::group::GroupsRepository;
use music_events_lib::model::person::PersonsRepository;
use music_events_lib::model::viewer::ViewersRepository;
use music_events_lib::model::viewer_seat::ViewerSeatsRepository;
use music_events_lib::services::events_service::events_service::*;
use music_events_lib::services::concerts_service::concerts_service::*;
use music_events_lib::services::groups_service::groups_service::*;
use music_events_lib::services::artists_service::artists_service::*;
use music_events_lib::services::persons_service::persons_service::*;
use music_events_lib::services::viewer_seats_service::viewer_seats_service::*;
use music_events_lib::services::viewers_service::viewers_service::*;
use tauri::Manager;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = Arc::new(establish_connection_pool().await.unwrap());
    let events_repository = EventsRepository::new(pool.clone());
    let concerts_repository = ConcertsRepository::new(pool.clone());
    let artists_repository = ArtistsRepository::new(pool.clone());
    let groups_repository = GroupsRepository::new(pool.clone());
    let viewer_seats_repository = ViewerSeatsRepository::new(pool.clone());
    let viewers_repository = ViewersRepository::new(pool.clone());
    let persons_repository = PersonsRepository::new(pool.clone());

    let transaction_storage = TransactionStorage::new();
    tauri::Builder::default()
        .manage(events_repository)
        .manage(concerts_repository)
        .manage(artists_repository)
        .manage(groups_repository)
        .manage(viewer_seats_repository)
        .manage(viewers_repository)
        .manage(persons_repository)
        .manage(transaction_storage)
        .invoke_handler(tauri::generate_handler![
            // event
            create_event,
            get_all_events,
            get_events_count,
            get_events_paginated,
            get_all_event_ids,
            get_event_by_id,
            add_event,
            update_event,
            remove_event,
            // concert
            create_concert,
            get_all_concerts,
            get_concerts_count,
            get_concerts_paginated,
            get_all_concert_ids,
            get_concert_by_id,
            add_concert,
            update_concert,
            remove_concert,
            // groups
            create_group,
            get_all_groups,
            get_groups_count,
            get_groups_paginated,
            get_all_group_ids,
            get_group_by_id,
            add_group,
            update_group,
            remove_group,
            // artists
            create_artist,
            get_all_artists,
            get_artists_count,
            get_artists_paginated,
            get_all_artist_ids,
            get_artist_by_id,
            add_artist,
            update_artist,
            remove_artist,
            // viewer_seat
            create_viewer_seat,
            get_all_viewer_seats,
            get_viewer_seats_count,
            get_viewer_seats_paginated,
            get_all_viewer_seat_ids,
            get_viewer_seat_by_id,
            add_viewer_seat,
            update_viewer_seat,
            remove_viewer_seat,
            // viewers
            create_viewer,
            get_all_viewers,
            get_viewers_count,
            get_viewers_paginated,
            get_all_viewer_ids,
            get_viewer_by_id,
            add_viewer,
            update_viewer,
            remove_viewer,
            // persons
            create_person,
            get_all_persons,
            get_persons_count,
            get_persons_paginated,
            get_all_person_ids,
            get_person_by_id,
            add_person,
            update_person,
            remove_person,
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
