use super::dao::group_entity::GroupEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "groups"]
#[entity = "GroupEntity"]
#[model = "Group"]
pub struct Group {
    id: Option<i64>,
    name: String,
    genre: String,
}

impl Group {
    pub fn new(name: String, genre: String) -> Group {
        Group {
            id: None,
            name,
            genre,
        }
    }
}

impl From<GroupEntity> for Group {
    fn from(group_entity: GroupEntity) -> Self {
        Group {
            id: Some(group_entity.id),
            name: group_entity.name,
            genre: group_entity.genre,
        }
    }
}
