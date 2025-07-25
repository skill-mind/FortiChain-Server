use crate::{AppState, Result, http::transaction::DepositRequest};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "deposit_handler", skip(state, payload))]
pub async fn deposit_handler(
    state: State<AppState>,
    Json(payload): Json<DepositRequest>,
) -> Result<StatusCode> {
    payload.validate()?;
    let mut tx = state.db.pool.begin().await?;

    sqlx::query!(
        r#"
        WITH balance_update AS (
            INSERT INTO escrow_users (wallet_address, balance)
            VALUES ($1, $2)
            ON CONFLICT (wallet_address) DO UPDATE
            SET balance = COALESCE(escrow_users.balance, 0) + EXCLUDED.balance
        )
        INSERT INTO escrow_transactions (
            wallet_address, amount, currency, transaction_hash, notes, type, status
        )
        VALUES ($1, $2, $3, $4, $5, 'deposit', 'completed')
        "#,
        payload.wallet_address,
        payload.amount,
        payload.currency,
        payload.transaction_hash,
        payload.notes
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(StatusCode::CREATED)
}
