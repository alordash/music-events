use super::dao::repertoire_entity::RepertoireEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "repertoires"]
#[entity = "RepertoireEntity"]
#[model = "Repertoire"]
pub struct Repertoire {
    id: Option<i64>,
    name: String,
    #[serde(rename(serialize = "orderNumber", deserialize = "orderNumber"))]
    order_number: i32,
    #[serde(rename(serialize = "participantId", deserialize = "participantId"))]
    participant_id: i64,
}

impl Repertoire {
    pub fn new(name: String, order_number: i32, participant_id: i64) -> Repertoire {
        Repertoire {
            id: None,
            name,
            order_number,
            participant_id,
        }
    }
}

impl From<RepertoireEntity> for Repertoire {
    fn from(repertoire_entity: RepertoireEntity) -> Self {
        Repertoire {
            id: Some(repertoire_entity.id),
            name: repertoire_entity.name,
            order_number: repertoire_entity.order_number,
            participant_id: repertoire_entity.participant_id,
        }
    }
}
