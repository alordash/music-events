use super::dao::event_entity::EventEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "events"]
#[entity = "EventEntity"]
#[model = "Event"]
pub struct Event {
    id: Option<i64>,
    name: String,
}

impl Event {
    pub fn new(name: String) -> Event {
        Event { id: None, name }
    }
}

impl From<EventEntity> for Event {
    fn from(event_entity: EventEntity) -> Self {
        Event {
            id: Some(event_entity.id),
            name: event_entity.name,
        }
    }
}
