use crate::model::viewer_seat::dao::viewer_seat_entity::ViewerSeatEntity;

use super::{ViewerSeat, ViewerSeatsRepository};
use sqlx::Error;

impl ViewerSeatsRepository {
    pub async fn get_concert_viewer_seats(
        &self,
        concert_id: u64,
    ) -> Result<Vec<ViewerSeat>, Error> {
        let viewer_seat_entities = sqlx::query_as!(
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

        Ok(viewer_seat_entities
            .into_iter()
            .map(ViewerSeat::from)
            .collect())
    }

    pub async fn get_free_viewer_seats_paginated(
        &self,
        count: i64,
        offset: i64,
    ) -> Result<Vec<ViewerSeat>, Error> {
        let viewer_seat_entities = sqlx::query_as!(
            ViewerSeatEntity,
            r#"
            SELECT vs.*
            FROM viewer_seats AS vs
            WHERE NOT EXISTS (
                SELECT v.id
                FROM viewers AS v
                WHERE v.viewer_seat_id = vs.id
            )
            ORDER BY id
            LIMIT $1
            OFFSET $2
            "#,
            count,
            offset
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        Ok(viewer_seat_entities
            .into_iter()
            .map(ViewerSeat::from)
            .collect())
    }

    pub async fn get_free_viewer_seats_count(&self) -> Result<u64, Error> {
        let (count,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM viewer_seats AS vs
            WHERE NOT EXISTS (
                SELECT v.id
                FROM viewers AS v
                WHERE v.viewer_seat_id = vs.id
            )
            "#,
        )
        .fetch_one(self.pool().as_ref())
        .await?;

        Ok(count as u64)
    }

    pub async fn get_free_concert_viewer_seats(
        &self,
        concert_id: u64,
    ) -> Result<Vec<ViewerSeat>, Error> {
        let viewer_seat_entities = sqlx::query_as!(
            ViewerSeatEntity,
            r#"
            SELECT *
            FROM viewer_seats AS vs
            WHERE concert_id = $1 AND NOT EXISTS (
                SELECT v.id
                FROM viewers AS v
                WHERE v.viewer_seat_id = vs.id
            )
            ORDER BY id
            "#,
            concert_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        Ok(viewer_seat_entities
            .into_iter()
            .map(ViewerSeat::from)
            .collect())
    }
}
