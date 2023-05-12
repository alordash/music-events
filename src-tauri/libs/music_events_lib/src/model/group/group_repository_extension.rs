use super::{dao::group_entity::GroupEntity, Group, GroupsRepository};
use sqlx::Error;

impl GroupsRepository {
    pub async fn get_concert_groups(&self, concert_id: u64) -> Result<Vec<Group>, Error> {
        let group_entities = sqlx::query_as!(
            GroupEntity,
            r#"
            SELECT grp.*
            FROM groups AS grp
            INNER JOIN participants AS participant
            ON participant.concert_id = $1
            AND participant.group_id = grp.id
            ORDER BY grp.id
            "#,
            concert_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        Ok(group_entities.into_iter().map(Group::from).collect())
    }
}
