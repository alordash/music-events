use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GroupArtistEntity {
    pub id: i64,
    pub group_id: i64,
    pub artist_id: i64,
    pub role: String
}
