use tauri::State;

use crate::{
    db::{
        general_controller,
        transaction_storage::{TransactionId, TransactionStorage},
    },
    services::db_error::db_error,
};

#[tauri::command]
pub async fn transaction_commit<'r, 't>(
    transaction_id: TransactionId,
    transaction_storage: State<'r, TransactionStorage<'t>>,
) -> Result<(), String> {
    let transaction_storage = &mut *transaction_storage.transactions.lock().await;

    let transaction = transaction_storage.remove(&transaction_id).ok_or(format!(
        "Transaction for id {:?} does not exist",
        transaction_id
    ))?;

    general_controller::transaction_commit(transaction)
        .await
        .map_err(db_error)?;
    Ok(())
}

#[tauri::command]
pub async fn transaction_rollback<'r, 't>(
    transaction_id: TransactionId,
    transaction_storage: State<'r, TransactionStorage<'t>>,
) -> Result<(), String> {
    let transaction_storage = &mut *transaction_storage.transactions.lock().await;

    let transaction = transaction_storage.remove(&transaction_id).ok_or(format!(
        "Transaction for id {:?} does not exist",
        transaction_id
    ))?;

    general_controller::transaction_rollback(transaction)
        .await
        .map_err(db_error)?;
    Ok(())
}
