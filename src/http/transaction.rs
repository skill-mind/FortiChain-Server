use crate::{
    AppState,
    services::transaction::{DepositRequest, TransactionService},
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/deposit", post(deposit))
}

#[tracing::instrument(skip(state, payload))]
pub async fn deposit(state: State<AppState>, Json(payload): Json<DepositRequest>) -> StatusCode {
    let transaction_service = TransactionService {};
    let deposit_transaction = transaction_service
        .deposit_funds(&state.db.pool, payload)
        .await;
    if deposit_transaction.is_ok() {
        StatusCode::CREATED
    } else {
        let (status_code, _) = From::from(deposit_transaction.err().unwrap());
        status_code
    }
}
