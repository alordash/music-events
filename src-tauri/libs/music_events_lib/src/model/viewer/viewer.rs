use super::dao::viewer_entity::ViewerEntity;
use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "viewers"]
#[entity = "ViewerEntity"]
#[model = "Viewer"]
pub struct Viewer {
    id: Option<i64>,
    #[serde(rename(serialize = "personId", deserialize = "personId"))]
    person_id: i64,
    #[serde(rename(serialize = "viewerSeatId", deserialize = "viewerSeatId"))]
    viewer_seat_id: i64,
}

impl Viewer {
    pub fn new(person_id: i64, viewer_seat_id: i64) -> Viewer {
        Viewer {
            id: None,
            person_id,
            viewer_seat_id,
        }
    }
}

impl From<ViewerEntity> for Viewer {
    fn from(viewer_entity: ViewerEntity) -> Self {
        Viewer {
            id: Some(viewer_entity.id),
            person_id: viewer_entity.person_id,
            viewer_seat_id: viewer_entity.viewer_seat_id,
        }
    }
}
