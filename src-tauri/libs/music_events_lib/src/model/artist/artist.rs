use super::dao::artist_entity::ArtistEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "artists"]
#[entity = "ArtistEntity"]
#[model = "Artist"]
pub struct Artist {
    id: Option<i64>,
    pseudonym: String,
    #[serde(rename(serialize = "personId", deserialize = "personId"))]
    person_id: i64,
}

impl Artist {
    pub fn new(pseudonym: String, person_id: i64) -> Artist {
        Artist {
            id: None,
            pseudonym,
            person_id,
        }
    }
}

impl From<ArtistEntity> for Artist {
    fn from(artist_entity: ArtistEntity) -> Self {
        Artist {
            id: Some(artist_entity.id),
            pseudonym: artist_entity.pseudonym,
            person_id: artist_entity.person_id,
        }
    }
}
