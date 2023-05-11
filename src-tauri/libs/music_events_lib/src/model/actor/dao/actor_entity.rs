use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ActorEntity {
    pub id: i64,
    pub pseudonym: String,
    pub person_id: i64
}
