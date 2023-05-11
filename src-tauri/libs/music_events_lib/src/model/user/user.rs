use super::dao::user_entity::UserEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "users"]
#[entity = "UserEntity"]
#[model = "User"]
pub struct User {
    id: Option<i64>,
    login: String,
    password: String,
    role: String,
}

impl User {
    pub fn new(login: String, password: String, role: String) -> User {
        User {
            id: None,
            login,
            password,
            role,
        }
    }
}

impl From<UserEntity> for User {
    fn from(user_entity: UserEntity) -> Self {
        User {
            id: Some(user_entity.id),
            login: user_entity.login,
            password: user_entity.password,
            role: user_entity.role,
        }
    }
}
