use super::dao::group_artist_entity::GroupArtistEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "group_artists"]
#[entity = "GroupArtistEntity"]
#[model = "GroupArtist"]
pub struct GroupArtist {
    id: Option<i64>,
    #[serde(rename(serialize = "groupId", deserialize = "groupId"))]
    group_id: i64,
    #[serde(rename(serialize = "artistId", deserialize = "artistId"))]
    artist_id: i64,
    role: String
}

impl GroupArtist {
    pub fn new(group_id: i64, artist_id: i64, role: String) -> GroupArtist {
        GroupArtist { id: None, group_id, artist_id, role }
    }
}

impl From<GroupArtistEntity> for GroupArtist {
    fn from(group_artist_entity: GroupArtistEntity) -> Self {
        GroupArtist {
            id: Some(group_artist_entity.id),
            group_id: group_artist_entity.group_id,
            artist_id: group_artist_entity.artist_id,
            role: group_artist_entity.role
        }
    }
}
