use super::dao::actor_entity::ActorEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "actors"]
#[entity = "ActorEntity"]
#[model = "Actor"]
pub struct Actor {
    id: Option<i64>,
    pseudonym: String,
    #[serde(rename(serialize = "personId", deserialize = "personId"))]
    person_id: i64,
}

impl Actor {
    pub fn new(pseudonym: String, person_id: i64) -> Actor {
        Actor {
            id: None,
            pseudonym,
            person_id,
        }
    }
}

impl From<ActorEntity> for Actor {
    fn from(actor_entity: ActorEntity) -> Self {
        Actor {
            id: Some(actor_entity.id),
            pseudonym: actor_entity.pseudonym,
            person_id: actor_entity.person_id,
        }
    }
}
