use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use tauri::async_runtime::Mutex;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum TransactionId {
    Concert(i64),
    ViewerSeat(i64),
}

pub struct TransactionStorage<'a> {
    pub transactions: Mutex<HashMap<TransactionId, Transaction<'a, Postgres>>>,
}

impl<'a> TransactionStorage<'a> {
    pub fn new() -> Self {
        TransactionStorage {
            transactions: Mutex::new(HashMap::new()),
        }
    }
}
