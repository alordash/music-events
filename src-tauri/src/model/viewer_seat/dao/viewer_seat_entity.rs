use serde::Serialize;
use sqlx::{postgres::PgRow, types::Decimal, FromRow, Row};

#[derive(Debug, Clone, Serialize)]
pub struct ViewerSeatEntity {
    pub id: i64,
    pub kind: String,
    pub cost_rubles: Decimal,
    pub real_number: i32,
    pub concert_id: i64,
}

impl<'r> FromRow<'r, PgRow> for ViewerSeatEntity {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let kind = row.try_get("kind")?;
        let cost_rubles = row.try_get("cost_rubles")?;
        let real_number = row.try_get("real_number")?;
        let concert_id = row.try_get("concert_id")?;

        Ok(ViewerSeatEntity {
            id,
            kind,
            cost_rubles,
            real_number,
            concert_id,
        })
    }
}
