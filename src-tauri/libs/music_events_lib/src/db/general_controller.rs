use sqlx::{Transaction, Postgres, Error};

pub async fn transaction_commit<'a>(
    transaction: Transaction<'a, Postgres>,
) -> Result<(), Error> {
    transaction.commit().await?;
    Ok(())
}

pub async fn transaction_rollback<'a>(
    transcation: Transaction<'a, Postgres>,
) -> Result<(), Error> {
    transcation.rollback().await?;
    Ok(())
}