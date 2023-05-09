use crate::model::viewer_seat::dao::viewer_seat_entity::ViewerSeatEntity;

use super::{ViewerSeat, ViewerSeatsRepository};
use sqlx::{Error, PgPool};

impl ViewerSeatsRepository {
    pub async fn get_concert_viewer_seats(
        &self,
        concert_id: u64,
    ) -> Result<Vec<ViewerSeat>, Error> {
        let concert_entities = sqlx::query_as!(
            ViewerSeatEntity,
            r#"
            SELECT *
            FROM viewer_seats
            WHERE concert_id = $1
            ORDER BY id
            "#,
            concert_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        Ok(concert_entities.into_iter().map(ViewerSeat::from).collect())
    }
}
