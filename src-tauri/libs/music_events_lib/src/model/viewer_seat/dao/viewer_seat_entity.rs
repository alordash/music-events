use serde::Serialize;
use sqlx::types::Decimal;

#[derive(Debug, Clone, Serialize)]
pub struct ViewerSeatEntity {
    pub id: i64,
    pub kind: String,
    pub cost_rubles: Decimal,
    pub real_number: i32,
    pub concert_id: i64,
}
