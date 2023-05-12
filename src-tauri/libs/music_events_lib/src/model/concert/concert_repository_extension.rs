use super::{dao::concert_entity::ConcertEntity, Concert, ConcertsRepository};
use sqlx::Error;

impl ConcertsRepository {
    pub async fn get_event_concerts(&self, event_id: u64) -> Result<Vec<Concert>, Error> {
        let concert_entities = sqlx::query_as!(
            ConcertEntity,
            r#"
            SELECT *
            FROM concerts
            WHERE event_id = $1
            ORDER BY id
            "#,
            event_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        Ok(concert_entities.into_iter().map(Concert::from).collect())
    }
}
