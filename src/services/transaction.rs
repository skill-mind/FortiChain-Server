use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::services::escrow::EscrowService;
use crate::services::utils::ServiceError;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    BountyAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub wallet_address: String,
    pub project_id: String,
    pub transaction_type: TransactionType,
    pub amount: u128,
    pub currency: String,
    pub transaction_hash: String,
    pub transaction_status: TransactionStatus,
    pub notes: Option<String>,
    pub created_at: f64,
    pub updated_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRequest {
    wallet_address: String,
    project_id: String,
    amount: u128,
    currency: String,
    notes: Option<String>,
    transaction_hash: String,
}

pub struct TransactionService {
    db: PgPool,
}

impl TransactionService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    pub async fn deposit_funds(
        &self,
        deposit_info: DepositRequest,
    ) -> Result<Transaction, ServiceError> {
        let mut tx = self.db.begin().await?;

        // Get or create escrow account
        let escrow_service = EscrowService::new(self.db);
        let escrow_account = escrow_service
            .get_or_create_escrow_users(deposit_info.wallet_address.clone())
            .await?;

        // Create transaction record
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;

        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            INSERT INTO escrow_transactions 
            (wallet_address, project_id, transaction_type, amount, currency, 
            transaction_hash, transaction_status, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, wallet_address, project_id, transaction_type as "transaction_type: TransactionType", 
                     amount, transaction_status as "transaction_status: TransactionStatus", description, created_at, updated_at
            "#,
            deposit_info.wallet_address,
            deposit_info.project_id,
            TransactionType::Deposit as TransactionType,
            deposit_info.amount,
            deposit_info.currency,
            deposit_info.transaction_hash,
            TransactionStatus::Pending as TransactionStatus,
            deposit_info.notes,
            now,
            now
        )
        .fetch_one(&mut *tx)
        .await?;

        // Update escrow account balance
        let new_balance = escrow_account.balance + deposit_info.amount;

        sqlx::query!(
            r#"
            UPDATE escrow_users
            SET balance = $1, updated_at = $2
            WHERE wallet_address = $3
            "#,
            new_balance,
            now,
            deposit_info.wallet_address
        )
        .execute(&mut *tx)
        .await?;

        // Update transaction status to completed
        sqlx::query!(
            r#"
            UPDATE escrow_transactions
            SET status = $1, updated_at = $2
            WHERE wallet_address = $3
            "#,
            TransactionStatus::Completed as TransactionStatus,
            now,
            deposit_info.wallet_address
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(transaction)
    }
}
