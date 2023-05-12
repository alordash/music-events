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
use music_events_lib::model::group_artist::GroupArtistsRepository;
use music_events_lib::model::participant::ParticipantsRepository;
use music_events_lib::model::person::PersonsRepository;
use music_events_lib::model::repertoire::RepertoiresRepository;
use music_events_lib::model::user::UsersRepository;
use music_events_lib::model::user_person::UserPersonsRepository;
use music_events_lib::model::viewer::ViewersRepository;
use music_events_lib::model::viewer_seat::ViewerSeatsRepository;

use music_events_lib::services::artists_service::artists_service::*;
use music_events_lib::services::concerts_service::concerts_service::*;
use music_events_lib::services::events_service::events_service::*;
use music_events_lib::services::group_artists_service::group_artists_service::*;
use music_events_lib::services::groups_service::groups_service::*;
use music_events_lib::services::participants_service::participants_service::*;
use music_events_lib::services::persons_service::persons_service::*;
use music_events_lib::services::repertoires_service::repertoires_service::*;
use music_events_lib::services::user_persons_service::user_persons_service::*;
use music_events_lib::services::users_service::users_service::*;
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
    let participants_repository = ParticipantsRepository::new(pool.clone());
    let repertoires_repository = RepertoiresRepository::new(pool.clone());
    let group_artists_repository = GroupArtistsRepository::new(pool.clone());
    let viewer_seats_repository = ViewerSeatsRepository::new(pool.clone());
    let viewers_repository = ViewersRepository::new(pool.clone());
    let persons_repository = PersonsRepository::new(pool.clone());
    let users_repository = UsersRepository::new(pool.clone());
    let user_persons_repository = UserPersonsRepository::new(pool.clone());

    let transaction_storage = TransactionStorage::new();
    tauri::Builder::default()
        .manage(events_repository)
        .manage(concerts_repository)
        .manage(artists_repository)
        .manage(groups_repository)
        .manage(participants_repository)
        .manage(repertoires_repository)
        .manage(group_artists_repository)
        .manage(viewer_seats_repository)
        .manage(viewers_repository)
        .manage(persons_repository)
        .manage(users_repository)
        .manage(user_persons_repository)
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
            get_event_concerts,
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
            get_concert_groups,
            // participants
            create_participant,
            get_all_participants,
            get_participants_count,
            get_participants_paginated,
            get_all_participant_ids,
            get_participant_by_id,
            add_participant,
            update_participant,
            remove_participant,
            // repertoires
            create_repertoire,
            get_all_repertoires,
            get_repertoires_count,
            get_repertoires_paginated,
            get_all_repertoire_ids,
            get_repertoire_by_id,
            add_repertoire,
            update_repertoire,
            remove_repertoire,
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
            get_group_artists,
            // group_artists
            create_group_artist,
            get_all_group_artists,
            get_group_artists_count,
            get_group_artists_paginated,
            get_all_group_artist_ids,
            get_group_artist_by_id,
            add_group_artist,
            update_group_artist,
            remove_group_artist,
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
            // users
            create_user,
            get_all_users,
            get_users_count,
            get_users_paginated,
            get_all_user_ids,
            get_user_by_id,
            add_user,
            update_user,
            remove_user,
            try_login_user,
            get_bought_viewer_seats,
            // user_persons
            create_user_person,
            get_all_user_persons,
            get_user_persons_count,
            get_user_persons_paginated,
            get_all_user_person_ids,
            get_user_person_by_id,
            add_user_person,
            update_user_person,
            remove_user_person
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
