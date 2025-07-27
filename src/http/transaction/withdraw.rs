use crate::{AppState, Error, Result, http::transaction::WithdrawalRequest};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "withdraw_handler", skip(state, payload))]
pub async fn withdraw_handler(
    state: State<AppState>,
    Json(payload): Json<WithdrawalRequest>,
) -> Result<StatusCode> {
    // Validate request payload
    payload.validate()?;

    let mut tx = state.db.pool.begin().await?;

    // First check if balance is sufficient
    let current_balance = sqlx::query!(
        r#"SELECT balance FROM escrow_users WHERE wallet_address = $1"#,
        payload.wallet_address
    )
    .fetch_optional(&mut *tx)
    .await?;

    let current_balance = match current_balance {
        Some(b) => b.balance,
        None => return Err(Error::NotFound),
    };

    if current_balance < payload.amount {
        return Err(Error::Conflict);
    }

    // Perform the withdrawal transaction atomically
    sqlx::query!(
        r#"
        WITH balance_update AS (
            UPDATE escrow_users
            SET balance = balance - $2
            WHERE wallet_address = $1
            RETURNING balance
        )
        INSERT INTO escrow_transactions (
            wallet_address, amount, currency, transaction_hash, notes, type, status
        )
        VALUES ($1, $2, $3, $4, $5, 'withdrawal', 'completed')
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
