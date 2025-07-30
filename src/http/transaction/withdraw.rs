use crate::{AppState, Error, Result, http::transaction::WithdrawalRequest};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "withdraw_handler", skip(state, payload))]
pub async fn withdraw_handler(
    state: State<AppState>,
    Json(payload): Json<WithdrawalRequest>,
) -> Result<StatusCode> {
    payload.validate()?;
    let mut tx = state.db.pool.begin().await?;

    let query_result = sqlx::query!(
        r#"
        WITH balance_update AS (
            UPDATE escrow_users
            SET balance = balance - $2
            WHERE wallet_address = $1 AND balance >= $2
            RETURNING 1
        )
        INSERT INTO escrow_transactions (
            wallet_address, amount, currency, transaction_hash, notes, type, status
        )
        SELECT $1, $2, $3, $4, $5, 'withdrawal', 'completed'
        FROM balance_update
        "#,
        payload.wallet_address,
        payload.amount,
        payload.currency,
        payload.transaction_hash,
        payload.notes
    )
    .execute(&mut *tx)
    .await?;

    match query_result.rows_affected() {
        1 => {
            tx.commit().await?;
            Ok(StatusCode::CREATED)
        }
        0 => {
            let exists: bool = sqlx::query_scalar(
                r#"
                SELECT EXISTS(SELECT 1 FROM escrow_users WHERE wallet_address = $1)
                "#,
            )
            .bind(payload.wallet_address)
            .fetch_one(&mut *tx)
            .await?;

            if exists {
                Err(Error::InvalidRequest("Insufficient Funds".to_string()))
            } else {
                Err(Error::NotFound)
            }
        }
        _ => {
            tracing::error!("Unexpected rows affected {}", query_result.rows_affected());
            Err(Error::InternalServerError(anyhow::anyhow!(
                "Unexpected rows affected"
            )))
        }
    }
}
