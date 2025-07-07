use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;

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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: String,
    pub wallet_address: String,
    pub project_id: String,
    pub transaction_type: TransactionType,
    pub amount: i64,
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
    amount: i64,
    currency: String,
    notes: Option<String>,
    transaction_hash: String,
}

pub struct TransactionService;

impl TransactionService {
    pub async fn deposit_funds(
        &self,
        db: &PgPool,
        deposit_info: DepositRequest,
    ) -> Result<Transaction, ServiceError> {
        let mut tx = db.begin().await?;

        // Get or create escrow account
        let escrow_service = EscrowService {};
        let escrow_account = escrow_service
            .get_or_create_escrow_users(db, deposit_info.wallet_address.clone())
            .await?;

        // Create transaction record
        let now = OffsetDateTime::now_utc();
        let query = r#"
            INSERT INTO escrow_transactions 
            (wallet_address, project_id, transaction_type, amount, currency, 
            transaction_hash, transaction_status, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, wallet_address, project_id, transaction_type as "transaction_type: TransactionType", 
                     amount, transaction_status as "transaction_status: TransactionStatus", description, created_at, updated_at
            "#;
        let transaction = 
            sqlx::query_as::<_,Transaction>(query)
            .bind(deposit_info.wallet_address.clone())
            .bind(deposit_info.project_id)
            .bind(TransactionType::Deposit as TransactionType)
            .bind(deposit_info.amount)
            .bind(deposit_info.currency)
            .bind(deposit_info.transaction_hash)
            .bind(TransactionStatus::Pending as TransactionStatus)
            .bind(deposit_info.notes)
            .bind(now)
            .bind(now)
        .fetch_one(&mut *tx)
        .await?;

        // Update escrow account balance
        let new_balance = escrow_account.balance + deposit_info.amount;

        sqlx::query(
            r#"
            UPDATE escrow_users
            SET balance = $1, updated_at = $2
            WHERE wallet_address = $3
            "#,)
            .bind(new_balance)
            .bind(now)
            .bind(deposit_info.wallet_address.clone())
        .execute(&mut *tx)
        .await?;

        // Update transaction status to completed
        sqlx::query(
            r#"
            UPDATE escrow_transactions
            SET status = $1, updated_at = $2
            WHERE wallet_address = $3
            "#,
        )
        .bind(TransactionStatus::Completed as TransactionStatus)
        .bind(now)
        .bind(deposit_info.wallet_address)
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(transaction)
    }
}
