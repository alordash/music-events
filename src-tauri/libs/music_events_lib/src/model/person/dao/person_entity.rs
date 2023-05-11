use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PersonEntity {
    pub id: i64,
    pub name: String,
    pub surname: String,
}
