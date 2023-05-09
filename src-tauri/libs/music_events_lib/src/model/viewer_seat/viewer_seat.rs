use crate::model::repository::Repository;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use sqlx::{types::Decimal, Error, PgPool};

use super::dao::viewer_seat_entity::ViewerSeatEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[derive(Repository)]
#[table_name = "viewer_seats"]
#[entity = "ViewerSeatEntity"]
#[model = "ViewerSeat"]
pub struct ViewerSeat {
    id: Option<i64>, // None if created manually, Some(id) if retrieved from db
    kind: String,
    // #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    #[serde(rename(serialize = "costRubles", deserialize = "costRubles"))]
    cost_rubles: Decimal,
    #[serde(rename(serialize = "realNumber", deserialize = "realNumber"))]
    real_number: i32,
    #[serde(rename(serialize = "concertId", deserialize = "concertId"))]
    concert_id: i64,
}

impl ViewerSeat {
    pub fn new(
        kind: String,
        cost_rubles: Decimal,
        real_number: i32,
        concert_id: i64,
    ) -> ViewerSeat {
        ViewerSeat {
            id: None,
            kind,
            cost_rubles,
            real_number,
            concert_id,
        }
    }
}

impl From<ViewerSeatEntity> for ViewerSeat {
    fn from(viewer_seat_entity: ViewerSeatEntity) -> Self {
        ViewerSeat {
            id: Some(viewer_seat_entity.id),
            kind: viewer_seat_entity.kind,
            cost_rubles: viewer_seat_entity.cost_rubles,
            real_number: viewer_seat_entity.real_number,
            concert_id: viewer_seat_entity.concert_id,
        }
    }
}
