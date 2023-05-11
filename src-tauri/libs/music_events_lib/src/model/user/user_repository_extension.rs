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
}
