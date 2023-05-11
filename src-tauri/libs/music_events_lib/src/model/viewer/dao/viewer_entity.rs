use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ViewerEntity {
    pub id: i64,
    pub person_id: i64,
    pub viewer_seat_id: i64
}
