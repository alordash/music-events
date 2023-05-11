use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EventEntity {
    pub id: i64,
    pub name: String,
}
