use crate::{
    AppState, Error, Result,
    http::escrow::{AllocateBountyRequest, generate_transaction_hash},
};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "allocate_bounty_handler", skip(state, payload))]
pub async fn allocate_bounty_handler(
    State(state): State<AppState>,
    Json(payload): Json<AllocateBountyRequest>,
) -> Result<StatusCode> {
    payload.validate()?;
    let tx_hash = generate_transaction_hash();
    let mut tx = state.db.pool.begin().await?;

    let result = sqlx::query_scalar!(
        r#"
        WITH escrow_update AS (
            UPDATE escrow_users
            SET balance = balance - $1,
                updated_at = NOW()
            WHERE wallet_address = $2
              AND balance >= $1
            RETURNING 1
        ),
        project_update AS (
            UPDATE projects
            SET bounty_amount = COALESCE(bounty_amount, 0) + $1,
                bounty_currency = $3,
                bounty_expiry_date = $4,
                updated_at = NOW()
            WHERE contract_address = $5
              AND EXISTS (SELECT 1 FROM escrow_update)
            RETURNING id
        ),
        transaction_insert AS (
            INSERT INTO escrow_transactions (
                wallet_address, project_id, type, amount, currency,
                transaction_hash, status, notes
            )
            SELECT $2, id, 'bounty_allocation', $1, $3, $6, 'completed', $7
            FROM project_update
        )
        SELECT COUNT(*) FROM escrow_update
        "#,
        payload.amount,
        payload.wallet_address,
        payload.currency,
        payload.bounty_expiry_date,
        payload.project_contract_address,
        tx_hash,
        "Bounty allocated to project via escrow"
    )
    .fetch_one(&mut *tx)
    .await?;

    if result.unwrap_or(0) == 0 {
        tracing::error!("Failed to allocate bounty: insufficient balance or project not found");
        tx.rollback().await?;
        return Err(Error::InvalidRequest(
            "Insufficient balance or project not found".to_string(),
        ));
    }

    tx.commit().await?;
    Ok(StatusCode::OK)
}
