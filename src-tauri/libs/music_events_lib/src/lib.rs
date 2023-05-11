#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

pub mod db;
pub mod model;
pub mod services;

pub use db::db_connection_pool::establish_connection_pool;
pub use db::transaction_storage::TransactionStorage;
pub use model::concert::ConcertsRepository;
pub use model::viewer_seat::ViewerSeatsRepository;
pub use services::concerts_service::concerts_service::*;
pub use services::transactions_service::transactions_service::*;
pub use services::viewer_seats_service::viewer_seats_service::*;
pub use tauri::Manager;
