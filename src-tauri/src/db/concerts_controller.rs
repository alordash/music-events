use sqlx::{Error, PgPool, Postgres, Transaction};

use crate::model::concert::{dao::concert_entity::ConcertEntity, Concert};

pub async fn get_all_concerts(pool: &PgPool) -> Result<Vec<Concert>, Error> {
    let concert_entities = sqlx::query_as!(
        ConcertEntity,
        r#"
        SELECT *
        FROM concerts
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(concert_entities.into_iter().map(Concert::from).collect())
}

pub async fn get_all_concert_ids(pool: &PgPool) -> Result<Vec<i64>, Error> {
    let concert_ids = sqlx::query!(
        r#"
        SELECT id
        FROM concerts
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(concert_ids.into_iter().map(|r| r.id).collect())
}

pub async fn get_all_concert_ids_and_names(pool: &PgPool) -> Result<Vec<(i64, String)>, Error> {
    let concert_ids_and_names = sqlx::query!(
        r#"
        SELECT id, name
        FROM concerts
        ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(concert_ids_and_names
        .into_iter()
        .map(|r| (r.id, r.name))
        .collect())
}

pub async fn get_concert_by_id(pool: &PgPool, concert_id: i64) -> Result<Option<Concert>, Error> {
    let concert_entity = sqlx::query_as!(
        ConcertEntity,
        r#"
        SELECT *
        FROM concerts
        WHERE id = $1
        "#,
        concert_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(concert_entity.map(Concert::from))
}

pub async fn add_concert(pool: &PgPool, concert: &Concert) -> Result<i64, Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO concerts (date, duration_minutes, address, name)
        VALUES ( $1, $2, $3, $4 )
        RETURNING id
        "#,
        concert.date(),
        concert.duration_minutes(),
        concert.address(),
        concert.name()
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn add_concert_transaction<'a>(
    pool: &PgPool,
    concert: &Concert,
) -> Result<(Transaction<'a, Postgres>, i64), Error> {
    let mut tx = pool.begin().await?;
    let rec = sqlx::query!(
        r#"
        INSERT INTO concerts (date, duration_minutes, address, name)
        VALUES ( $1, $2, $3, $4 )
        RETURNING id
    "#,
        concert.date(),
        concert.duration_minutes(),
        concert.address(),
        concert.name(),
    )
    .fetch_one(&mut tx)
    .await?;

    Ok((tx, rec.id))
}

pub async fn update_concert(pool: &PgPool, concert: &Concert) -> Result<(), Error> {
    if concert.id().is_none() {
        return Err(Error::RowNotFound);
    }
    sqlx::query!(
        r#"
        UPDATE concerts 
        SET
        date = $1,
        duration_minutes = $2,
        address = $3,
        name = $4
        WHERE id = $5
        "#,
        concert.date(),
        concert.duration_minutes(),
        concert.address(),
        concert.name(),
        concert.id().unwrap()
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn remove_concert(pool: &PgPool, concert_id: i64) -> Result<u64, Error> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM concerts
        WHERE id = $1
        "#,
        concert_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected)
}
