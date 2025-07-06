use axum::{extract::State, Json};
use crate::{services::{transaction::{DepositRequest, Transaction, TransactionService}, utils::ServiceError}, AppState};

pub async fn deposit(
    State(state): State<AppState>,
    Json(payload): Json<DepositRequest>,
) -> Result<Json<Transaction>, ServiceError> {

    let transaction_service = TransactionService::new(state.db.pool);
    let deposit_transaction = transaction_service.deposit_funds(payload)
    .await?;
    Ok(Json(deposit_transaction))
}