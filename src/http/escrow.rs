use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
// use sqlx::Acquire;
use super::helpers::generate_transaction_hash;
use super::types::AllocateBountyRequest;
use crate::AppState;
use bigdecimal::{BigDecimal, Zero};

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/allocate_bounty", post(allocate_bounty_handler))
}

#[tracing::instrument(name = "allocate_bounty_handler", skip(state, payload))]
async fn allocate_bounty_handler(
    State(state): State<AppState>,
    Json(payload): Json<AllocateBountyRequest>,
) -> StatusCode {
    let db = &state.db;
    // Validate amount
    if payload.amount <= BigDecimal::zero() {
        tracing::error!("Bounty amount must be positive");
        return StatusCode::BAD_REQUEST;
    }
    // Validate address format (0x + 64 hex chars)
    let is_valid_addr = |addr: &str| {
        addr.starts_with("0x")
            && addr.len() == 66
            && addr.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    };
    if !is_valid_addr(&payload.wallet_address) || !is_valid_addr(&payload.project_contract_address)
    {
        tracing::error!("Invalid address format");
        return StatusCode::BAD_REQUEST;
    }
    // Start transaction
    tracing::info!("Starting transaction for bounty allocation");
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start bounty allocation transaction: {e:?}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    // 1. Lock and check escrow balance
    let escrow_row = sqlx::query!(
        "SELECT balance FROM escrow_users WHERE wallet_address = $1 FOR UPDATE",
        payload.wallet_address
    )
    .fetch_optional(&mut *tx)
    .await;
    let balance = match escrow_row {
        Ok(Some(row)) => row.balance,
        Ok(None) => {
            tracing::error!("Escrow user not found");
            let _ = tx.rollback().await;
            return StatusCode::BAD_REQUEST;
        }
        Err(e) => {
            tracing::error!("Failed to fetch escrow user: {e:?}");
            let _ = tx.rollback().await;
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    if balance < payload.amount {
        tracing::error!("Insufficient escrow balance");
        let _ = tx.rollback().await;
        return StatusCode::BAD_REQUEST;
    }
    // 2. Fetch project and validate ownership
    let project_row = sqlx::query!(
        "SELECT id, owner_address, bounty_amount FROM projects WHERE contract_address = $1 FOR UPDATE",
        payload.project_contract_address
    )
    .fetch_optional(&mut *tx)
    .await;
    let (project_id, owner_address, current_bounty) = match project_row {
        Ok(Some(row)) => (
            row.id,
            row.owner_address,
            row.bounty_amount.unwrap_or(BigDecimal::zero()),
        ),
        Ok(None) => {
            tracing::error!("Project not found");
            let _ = tx.rollback().await;
            return StatusCode::BAD_REQUEST;
        }
        Err(e) => {
            tracing::error!("Failed to fetch project: {e:?}");
            let _ = tx.rollback().await;
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    if owner_address != payload.wallet_address {
        tracing::error!("User does not own the project");
        let _ = tx.rollback().await;
        return StatusCode::FORBIDDEN;
    }
    // 3. Deduct balance
    let update_balance = sqlx::query!(
        "UPDATE escrow_users SET balance = balance - $1, updated_at = NOW() WHERE wallet_address = $2",
        payload.amount,
        payload.wallet_address
    )
    .execute(&mut *tx)
    .await;
    if let Err(e) = update_balance {
        tracing::error!("Failed to update escrow balance: {e:?}");
        let _ = tx.rollback().await;
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    // 4. Update project bounty fields
    let expiry = payload.bounty_expiry_date;

    let update_project = sqlx::query!(
        "UPDATE projects SET bounty_amount = $1, bounty_currency = $2, bounty_expiry_date = $3, updated_at = NOW() WHERE id = $4",
        current_bounty + &payload.amount,
        payload.currency,
        expiry,
        project_id
    )
    .execute(&mut *tx)
    .await;
    if let Err(e) = update_project {
        tracing::error!("Failed to update project bounty: {e:?}");
        let _ = tx.rollback().await;
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    // 5. Insert escrow transaction
    let tx_hash = generate_transaction_hash();

    let insert_tx = sqlx::query!(
        "INSERT INTO escrow_transactions (wallet_address, project_id, type, amount, currency, transaction_hash, status, notes) VALUES ($1, $2, 'bounty_allocation', $3, $4, $5, 'completed', $6)",
        payload.wallet_address,
        project_id,
        payload.amount,
        payload.currency,
        tx_hash,
        Some("Bounty allocated to project via escrow")
    )
    .execute(&mut *tx)
    .await;
    if let Err(e) = insert_tx {
        tracing::error!("Failed to insert escrow transaction: {e:?}");
        let _ = tx.rollback().await;
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    // Commit
    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {e:?}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}
