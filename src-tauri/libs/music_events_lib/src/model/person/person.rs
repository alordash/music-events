use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use super::dao::person_entity::PersonEntity;
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "persons"]
#[entity = "PersonEntity"]
#[model = "Person"]
pub struct Person {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    name: String,
    surname: String,
}

impl Person {
    pub fn new(name: String, surname: String) -> Person {
        Person {
            id: None,
            name,
            surname,
        }
    }
}

impl From<PersonEntity> for Person {
    fn from(person_entity: PersonEntity) -> Self {
        Person {
            id: Some(person_entity.id),
            name: person_entity.name,
            surname: person_entity.surname,
        }
    }
}
