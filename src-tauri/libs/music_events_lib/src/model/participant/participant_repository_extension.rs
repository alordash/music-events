use crate::model::repertoire::{dao::repertoire_entity::RepertoireEntity, Repertoire};

use super::ParticipantsRepository;
use sqlx::Error;

impl ParticipantsRepository {
    pub async fn get_participant_repertoires(
        &self,
        participant_id: u64,
    ) -> Result<Vec<Repertoire>, Error> {
        let participant_entities = sqlx::query_as!(
            RepertoireEntity,
            r#"
            SELECT *
            FROM repertoires
            WHERE participant_id = $1
            ORDER BY id
            "#,
            participant_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        Ok(participant_entities
            .into_iter()
            .map(Repertoire::from)
            .collect())
    }
}
