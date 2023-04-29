use chrono::prelude::*;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use super::dao::concert_entity::ConcertEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Concert {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    date: DateTime<Utc>,
    #[serde(rename(serialize = "durationMinutes", deserialize = "durationMinutes"))]
    duration_minutes: i32,
    address: String,
    name: String,
}

impl Concert {
    pub fn new(
        date: DateTime<Utc>,
        duration_minutes: i32,
        address: String,
        name: String,
    ) -> Concert {
        Concert {
            id: None,
            date,
            duration_minutes,
            address,
            name,
        }
    }
}

impl From<ConcertEntity> for Concert {
    fn from(concert_entity: ConcertEntity) -> Self {
        Concert {
            id: Some(concert_entity.id),
            date: concert_entity.date,
            duration_minutes: concert_entity.duration_minutes,
            address: concert_entity.address,
            name: concert_entity.name,
        }
    }
}
