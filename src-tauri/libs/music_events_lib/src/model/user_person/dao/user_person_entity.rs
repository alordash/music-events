use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserPersonEntity {
    pub id: i64,
    pub user_id: i64,
    pub person_id: i64
}
