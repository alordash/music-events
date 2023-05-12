use crate::model::{
    person::{dao::person_entity::PersonEntity, Person},
    viewer::{dao::viewer_entity::ViewerEntity, Viewer},
    viewer_seat::{dao::viewer_seat_entity::ViewerSeatEntity, ViewerSeat},
};

use super::{dao::user_entity::UserEntity, User, UsersRepository};
use sqlx::Error;

impl UsersRepository {
    pub async fn try_login(&self, login: String, password: String) -> Result<Option<User>, Error> {
        let maybe_user_entity = sqlx::query_as!(
            UserEntity,
            r#"
            SELECT *
            FROM users
            WHERE login = $1
            AND password = $2
            ORDER BY id
            "#,
            login,
            password
        )
        .fetch_optional(self.pool().as_ref())
        .await?;

        Ok(maybe_user_entity.map(User::from))
    }

    pub async fn get_bought_viewer_seats(
        &self,
        user_id: u64,
    ) -> Result<Vec<(Person, Viewer, ViewerSeat)>, Error> {
        let person_entities = sqlx::query_as!(
            PersonEntity,
            r#"
            SELECT person.*
            FROM users AS usr
            INNER JOIN user_persons AS user_person
            ON user_person.user_id = usr.id
            INNER JOIN persons AS person
            ON user_person.person_id = person.id
            INNER JOIN viewers AS viewer
            ON viewer.person_id = person.id
            INNER JOIN viewer_seats AS viewer_seat
            ON viewer.viewer_seat_id = viewer_seat.id
            WHERE usr.id = $1;
            "#,
            user_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        let viewer_entities = sqlx::query_as!(
            ViewerEntity,
            r#"
                SELECT viewer.*
                FROM users AS usr
                INNER JOIN user_persons AS user_person
                ON user_person.user_id = usr.id
                INNER JOIN persons AS person
                ON user_person.person_id = person.id
                INNER JOIN viewers AS viewer
                ON viewer.person_id = person.id
                INNER JOIN viewer_seats AS viewer_seat
                ON viewer.viewer_seat_id = viewer_seat.id
                WHERE usr.id = $1;
                "#,
            user_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        let viewer_seat_entities = sqlx::query_as!(
            ViewerSeatEntity,
            r#"
            SELECT viewer_seat.*
            FROM users AS usr
            INNER JOIN user_persons AS user_person
            ON user_person.user_id = usr.id
            INNER JOIN persons AS person
            ON user_person.person_id = person.id
            INNER JOIN viewers AS viewer
            ON viewer.person_id = person.id
            INNER JOIN viewer_seats AS viewer_seat
            ON viewer.viewer_seat_id = viewer_seat.id
            WHERE usr.id = $1;
            "#,
            user_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        let persons_viewer_viewer_seats: Vec<(Person, Viewer, ViewerSeat)> = person_entities
            .into_iter()
            .map(Person::from)
            .zip(viewer_entities.into_iter().map(Viewer::from))
            .zip(viewer_seat_entities.into_iter().map(ViewerSeat::from))
            .map(|((p, v), vs)| (p, v, vs))
            .collect();

        Ok(persons_viewer_viewer_seats)
    }
}
