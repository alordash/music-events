use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RepertoireEntity {
    pub id: i64,
    pub name: String,
    pub order_number: i32,
    pub participant_id: i64
}
