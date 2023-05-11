use super::dao::participant_entity::ParticipantEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "participants"]
#[entity = "ParticipantEntity"]
#[model = "Participant"]
pub struct Participant {
    id: Option<i64>,
    #[serde(rename(serialize = "concertId", deserialize = "concertId"))]
    concert_id: i64,
    #[serde(rename(serialize = "groupId", deserialize = "groupId"))]
    group_id: i64,
}

impl Participant {
    pub fn new(concert_id: i64, group_id: i64) -> Participant {
        Participant {
            id: None,
            concert_id,
            group_id,
        }
    }
}

impl From<ParticipantEntity> for Participant {
    fn from(participant_entity: ParticipantEntity) -> Self {
        Participant {
            id: Some(participant_entity.id),
            concert_id: participant_entity.concert_id,
            group_id: participant_entity.group_id,
        }
    }
}
