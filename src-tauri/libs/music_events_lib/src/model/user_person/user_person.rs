use super::dao::user_person_entity::UserPersonEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "user_persons"]
#[entity = "UserPersonEntity"]
#[model = "UserPerson"]
pub struct UserPerson {
    id: Option<i64>,
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    user_id: i64,
    #[serde(rename(serialize = "personId", deserialize = "personId"))]
    person_id: i64,
}

impl UserPerson {
    pub fn new(user_id: i64, person_id: i64) -> UserPerson {
        UserPerson {
            id: None,
            user_id,
            person_id,
        }
    }
}

impl From<UserPersonEntity> for UserPerson {
    fn from(user_person_entity: UserPersonEntity) -> Self {
        UserPerson {
            id: Some(user_person_entity.id),
            user_id: user_person_entity.user_id,
            person_id: user_person_entity.person_id,
        }
    }
}
