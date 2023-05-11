use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GroupEntity {
    pub id: i64,
    pub name: String,
    pub genre: String
}
