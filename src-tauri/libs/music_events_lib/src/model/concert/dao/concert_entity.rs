use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ConcertEntity {
    pub id: i64,
    pub date: NaiveDateTime,
    pub duration_minutes: i32,
    pub address: String,
    pub name: String,
    pub event_id: i64
}
