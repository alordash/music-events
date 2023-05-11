use crate::model::repository::Repository;
use chrono::prelude::*;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use super::dao::concert_entity::ConcertEntity;
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "concerts"]
#[entity = "ConcertEntity"]
#[model = "Concert"]
pub struct Concert {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    #[serde(
        deserialize_with = "crate::model::date_time_custom_serde::deserialize_naive_date_time"
    )]
    #[serde(serialize_with = "crate::model::date_time_custom_serde::serialize_naive_date_time")]
    date: NaiveDateTime,
    #[serde(rename(serialize = "durationMinutes", deserialize = "durationMinutes"))]
    duration_minutes: i32,
    address: String,
    name: String,
    #[serde(rename(serialize = "eventId", deserialize = "eventId"))]
    event_id: i64,
}

impl Concert {
    pub fn new(
        date: NaiveDateTime,
        duration_minutes: i32,
        address: String,
        name: String,
        event_id: i64,
    ) -> Concert {
        Concert {
            id: None,
            date,
            duration_minutes,
            address,
            name,
            event_id,
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
            event_id: concert_entity.event_id,
        }
    }
}
