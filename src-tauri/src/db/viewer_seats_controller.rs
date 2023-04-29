use sqlx::{postgres::PgRow, Error, Executor, FromRow, PgPool, Postgres, Transaction};

use crate::model::viewer_seat::{
    dao::viewer_seat_entity::ViewerSeatEntity, viewer_seat::ViewerSeat,
};

pub async fn get_n_viewer_seats_with_offset(
    pool: &PgPool,
    count: u32,
    offset: i32,
) -> Result<Vec<ViewerSeat>, Error> {
    let mut tx = pool.begin().await?;
    tx.execute(
        format!(
            r#"
        DECLARE cursor_viewer_seats CURSOR FOR SELECT * FROM viewer_seats;
        MOVE FORWARD {} FROM cursor_viewer_seats;
        "#,
            offset
        )
        .as_str(),
    )
    .await?;

    let pg_rows = sqlx::query(format!("FETCH {} FROM cursor_viewer_seats", count).as_str())
        .fetch_all(&mut tx)
        .await?;
    let viewer_seat_entities: Vec<ViewerSeatEntity> = pg_rows
        .into_iter()
        .map(|v| <ViewerSeatEntity as FromRow<PgRow>>::from_row(&v).unwrap())
        .collect();
    tx.commit().await?;

    Ok(viewer_seat_entities
        .into_iter()
        .map(ViewerSeat::from)
        .collect())
}

pub async fn get_all_viewer_seats(pool: &PgPool) -> Result<Vec<ViewerSeat>, Error> {
    let viewer_seat_entities = sqlx::query_as!(
        ViewerSeatEntity,
        r#"
        SELECT *
        FROM viewer_seats
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(viewer_seat_entities
        .into_iter()
        .map(ViewerSeat::from)
        .collect())
}

pub async fn get_all_viewer_seat_ids_and_real_numbers_and_concert_names(
    pool: &PgPool,
) -> Result<Vec<(i64, i32, String)>, Error> {
    let viewer_seat_ids_and_real_numbers = sqlx::query!(
        r#"
        SELECT vs.id, vs.real_number, c.name
        FROM viewer_seats AS vs
        LEFT JOIN concerts AS c ON vs.concert_id = c.id
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(viewer_seat_ids_and_real_numbers
        .into_iter()
        .map(|r| (r.id, r.real_number, r.name))
        .collect())
}

pub async fn get_viewer_seat_by_id(
    pool: &PgPool,
    viewer_seat_id: i64,
) -> Result<Option<ViewerSeat>, Error> {
    let viewer_seat_entity = sqlx::query_as!(
        ViewerSeatEntity,
        r#"
        SELECT *
        FROM viewer_seats
        WHERE id = $1
        LIMIT 1
        "#,
        viewer_seat_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(viewer_seat_entity.map(ViewerSeat::from))
}

pub async fn add_viewer_seat(pool: &PgPool, viewer_seat: &ViewerSeat) -> Result<i64, Error> {
    if viewer_seat.kind().is_empty() {
        return Err(Error::RowNotFound);
    }
    let rec = sqlx::query!(
        r#"
        INSERT INTO viewer_seats (kind, cost_rubles, real_number, concert_id)
        VALUES ( $1, $2, $3, $4 )
        RETURNING id
        "#,
        viewer_seat.kind(),
        viewer_seat.cost_rubles(),
        viewer_seat.real_number(),
        viewer_seat.concert_id()
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn update_viewer_seat(pool: &PgPool, viewer_seat: &ViewerSeat) -> Result<(), Error> {
    if viewer_seat.id().is_none() {
        return Err(Error::RowNotFound);
    }
    sqlx::query!(
        r#"
        UPDATE viewer_seats 
        SET
        kind = $1,
        cost_rubles = $2,
        real_number = $3,
        concert_id = $4
        WHERE id = $5
        "#,
        viewer_seat.kind(),
        viewer_seat.cost_rubles(),
        viewer_seat.real_number(),
        viewer_seat.concert_id(),
        viewer_seat.id().unwrap()
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

pub async fn update_viewer_seat_transaction<'a>(
    pool: &PgPool,
    viewer_seat: &ViewerSeat,
) -> Result<Transaction<'a, Postgres>, Error> {
    if viewer_seat.id().is_none() {
        return Err(Error::RowNotFound);
    }

    let mut tx = pool.begin().await?;
    sqlx::query!(
        r#"
        UPDATE viewer_seats 
        SET
        kind = $1,
        cost_rubles = $2,
        real_number = $3,
        concert_id = $4
        WHERE id = $5
        "#,
        viewer_seat.kind(),
        viewer_seat.cost_rubles(),
        viewer_seat.real_number(),
        viewer_seat.concert_id(),
        viewer_seat.id().unwrap()
    )
    .execute(&mut tx)
    .await?;

    Ok(tx)
}

pub async fn remove_viewer_seat(pool: &PgPool, viewer_seat_id: i64) -> Result<u64, Error> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM viewer_seats
        WHERE id = $1
        "#,
        viewer_seat_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected)
}

pub async fn remove_viewer_seat_transaction<'a>(
    pool: &PgPool,
    viewer_seat_id: i64,
) -> Result<Transaction<'a, Postgres>, Error> {
    let mut tx = pool.begin().await?;
    sqlx::query!("DELETE FROM viewer_seats WHERE id = $1", viewer_seat_id)
        .execute(&mut tx)
        .await?;

    Ok(tx)
}

pub async fn get_concert_viewer_seats(
    pool: &PgPool,
    concert_id: i64,
) -> Result<Vec<ViewerSeat>, Error> {
    let concert_entities = sqlx::query_as!(
        ViewerSeatEntity,
        r#"
        SELECT *
        FROM viewer_seats
        WHERE concert_id = $1
        ORDER BY id
        "#,
        concert_id
    )
    .fetch_all(pool)
    .await?;

    Ok(concert_entities.into_iter().map(ViewerSeat::from).collect())
}
