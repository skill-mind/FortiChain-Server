use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
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

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub wallet_address: String,
    pub project_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub amount: BigDecimal,
    pub currency: String,
    pub transaction_hash: String,
    pub transaction_status: TransactionStatus,
    pub notes: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRequest {
    wallet_address: String,
    amount: i64,
    currency: String,
    notes: Option<String>,
    transaction_hash: String,
}

#[derive(Debug, Clone)]
pub struct TransactionService;

impl TransactionService {
    #[tracing::instrument(skip(db))]
    pub async fn deposit_funds(
        &self,
        db: &PgPool,
        deposit_info: DepositRequest,
    ) -> Result<Transaction, ServiceError> {
        let mut tx = db.begin().await?;

        // Get or create escrow account
        let escrow_service = EscrowService {};
        let escrow_account = escrow_service
            .get_or_create_escrow_users(db, deposit_info.wallet_address.as_str())
            .await?;

        // Create transaction record
        tracing::info!("Creating Deposit Transaction ");
        let now = OffsetDateTime::now_utc();
        let query = r#"
            INSERT INTO escrow_transactions
            (wallet_address, type, amount, currency, transaction_hash, status, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
            id, wallet_address, project_id, type AS "transaction_type", amount, currency,
            transaction_hash, status AS "transaction_status", notes, created_at, updated_at
            "#;

        let transaction = sqlx::query_as::<_, Transaction>(query)
            .bind(deposit_info.wallet_address.clone())
            .bind(TransactionType::Deposit)
            .bind(BigDecimal::from(deposit_info.amount))
            .bind(deposit_info.currency)
            .bind(deposit_info.transaction_hash)
            .bind(TransactionStatus::Completed)
            .bind(deposit_info.notes)
            .bind(now)
            .bind(now)
            .fetch_one(&mut *tx)
            .await;

        if let Err(e) = transaction {
            tracing::error!(error = %e, "Failed to create deposit transaction");
            return Err(ServiceError::DatabaseError(e));
        }
        let transaction = transaction.unwrap();

        tracing::info!("Updating escrow account balance");
        // Update escrow account balance
        let new_balance = escrow_account.balance + BigDecimal::from(deposit_info.amount);

        match sqlx::query(
            r#"
            UPDATE escrow_users
            SET balance = $1, updated_at = $2
            WHERE wallet_address = $3
            "#,
        )
        .bind(new_balance)
        .bind(now)
        .bind(deposit_info.wallet_address.clone())
        .execute(&mut *tx)
        .await
        {
            Ok(_) => tracing::info!("Escrow account balance updated successfully"),
            Err(e) => {
                tracing::error!(error = %e, "Failed to update escrow account balance");
                return Err(ServiceError::DatabaseError(e));
            }
        }

        // Commit transaction
        match tx.commit().await {
            Ok(_) => tracing::info!("Transaction committed successfully"),
            Err(e) => {
                tracing::error!(error = %e, "Failed to commit transaction");
                return Err(ServiceError::DatabaseError(e));
            }
        }

        tracing::info!("Deposit transaction completed successfully");

        Ok(transaction)
    }
}
