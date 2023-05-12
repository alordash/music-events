use crate::model::user_person::dao::user_person_entity::UserPersonEntity;

use super::{UserPerson, UserPersonsRepository};
use sqlx::Error;

impl UserPersonsRepository {
    pub async fn get_user_person_by_user_id_and_person_id(
        &self,
        user_id: u64,
        person_id: u64,
    ) -> Result<Option<UserPerson>, Error> {
        let maybe_user_person_entity = sqlx::query_as!(
            UserPersonEntity,
            r#"
            SELECT *
            FROM user_persons
            WHERE user_id = $1
            AND person_id = $2
            ORDER BY id
            "#,
            user_id as i64,
            person_id as i64
        )
        .fetch_optional(self.pool().as_ref())
        .await?;

        Ok(maybe_user_person_entity.map(UserPerson::from))
    }

    pub async fn remove_user_person_by_person_id(&self, person_id: u64) -> Result<u64, Error> {
        let rows_affected = sqlx::query!(
            "DELETE FROM user_persons WHERE person_id = $1",
            person_id as i64
        )
        .execute(self.pool().as_ref())
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}
