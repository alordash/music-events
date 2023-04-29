use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ConcertEntity {
    pub id: i64,
    pub date: DateTime<Utc>,
    pub duration_minutes: i32,
    pub address: String,
    pub name: String
}
