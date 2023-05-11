use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ParticipantEntity {
    pub id: i64,
    pub concert_id: i64,
    pub group_id: i64
}
