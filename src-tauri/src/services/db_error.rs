pub fn db_error(sqlx_error: sqlx::Error) -> String {
    format!("DB error: {}", sqlx_error)
}