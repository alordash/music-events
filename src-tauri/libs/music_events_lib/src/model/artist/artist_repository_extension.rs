use super::{dao::artist_entity::ArtistEntity, Artist, ArtistsRepository};
use sqlx::Error;

impl ArtistsRepository {
    pub async fn get_group_artists(&self, group_id: u64) -> Result<Vec<Artist>, Error> {
        let artist_entities = sqlx::query_as!(
            ArtistEntity,
            r#"
            SELECT art.*
            FROM artists AS art
            INNER JOIN group_artists AS grp_art
            ON grp_art.group_id = $1
            AND grp_art.artist_id = art.id
            ORDER BY art.id
            "#,
            group_id as i64
        )
        .fetch_all(self.pool().as_ref())
        .await?;

        Ok(artist_entities.into_iter().map(Artist::from).collect())
    }
}
