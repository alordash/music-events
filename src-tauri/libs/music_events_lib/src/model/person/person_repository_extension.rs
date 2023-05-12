use crate::model::person::dao::person_entity::PersonEntity;

use super::{Person, PersonsRepository};
use sqlx::Error;

impl PersonsRepository {
    pub async fn get_person_by_name_and_surname(
        &self,
        name: String,
        surname: String,
    ) -> Result<Option<Person>, Error> {
        let maybe_person_entity = sqlx::query_as!(
            PersonEntity,
            r#"
            SELECT *
            FROM persons
            WHERE name = $1
            AND surname = $2
            ORDER BY id
            "#,
            name,
            surname
        )
        .fetch_optional(self.pool().as_ref())
        .await?;

        Ok(maybe_person_entity.map(Person::from))
    }
}
