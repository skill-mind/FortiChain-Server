use crate::{AppState, };
use crate::error::ServiceError;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use crate::http::helpers::{
    generate_transaction_hash, check_withdrawal_amount, check_withdrawal_amount_as_against_balance
};

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
pub struct EscrowUsers {
    pub wallet_address: String,
    pub balance: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct EscrowService;

impl EscrowService {
    /// Create or get existing escrow account for user
    #[tracing::instrument(skip(tx))]
    pub async fn get_or_create_escrow_users(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_wallet: &str,
    ) -> Result<EscrowUsers, ServiceError> {
        tracing::info!(wallet = %user_wallet, "Checking for existing escrow account");

        let query = r#"
            SELECT wallet_address, balance, created_at, updated_at
            FROM escrow_users
            WHERE wallet_address = $1;
        "#;

        let existing_account = sqlx::query_as::<_, EscrowUsers>(query)
            .bind(user_wallet)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to fetch existing escrow account");
                ServiceError::DatabaseError(e)
            })?;

        if let Some(account) = existing_account {
            return Ok(account);
        }

        tracing::info!(wallet = %user_wallet, "No existing escrow account found, creating a new one");

        let create_account_query = r#"
            INSERT INTO escrow_users (wallet_address, created_at, updated_at)
            VALUES ($1, $2, $3)
            RETURNING wallet_address, balance, created_at, updated_at;
        "#;
        let now = Utc::now();
        let new_account = sqlx::query_as::<_, EscrowUsers>(create_account_query)
            .bind(user_wallet)
            .bind(now)
            .bind(now)
            .fetch_one(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to create new escrow account");
                ServiceError::DatabaseError(e)
            })?;

        tracing::info!(wallet = %new_account.wallet_address, "New escrow account created successfully");

        Ok(new_account)
    }

    #[tracing::instrument(skip(tx))]
    pub async fn get_escrow_user(&self, tx: &mut Transaction<'_, Postgres>, wallet_address: &String) -> Result<EscrowUsers, ServiceError> {

        // Get escrow user with lock for update
        let query = r#"
        SELECT wallet_address, balance, created_at, updated_at
        FROM escrow_users
        WHERE wallet_address = $1
        FOR UPDATE;
        "#;

        let user = sqlx::query_as::<_, EscrowUsers>(query)
            .bind(&wallet_address)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to fetch escrow account");
                ServiceError::DatabaseError(e)
            })?
            .ok_or(ServiceError::EntityNotFound)?;
        Ok(user)
    }

    #[tracing::instrument(skip(tx))]
    pub async fn update_escrow_user_balance(&self, tx: &mut Transaction<'_, Postgres>, wallet_address: &String, new_balance: &BigDecimal) -> Result<(), ServiceError> {

        let update_query = r#"
        UPDATE escrow_users
        SET balance = $1, updated_at = $2
        WHERE wallet_address = $3;
    "#;

        let current_date_time = Utc::now();
        sqlx::query(update_query)
            .bind(&new_balance)
            .bind(current_date_time)
            .bind(wallet_address)
            .execute(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to update escrow account balance");
                ServiceError::DatabaseError(e)
            })?;

        Ok(())
    }
}

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

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct EscrowTransaction {
    pub id: Uuid,
    pub wallet_address: String,
    pub project_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub amount: BigDecimal,
    pub currency: String,
    pub transaction_hash: String,
    pub transaction_status: TransactionStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRequest {
    wallet_address: String,
    amount: i64,
    currency: String,
    notes: Option<String>,
    transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    wallet_address: String,
    amount: BigDecimal,
    currency: String,
    notes: Option<String>, // to tell the purpose of the withdrawal
}

#[derive(Debug, Clone)]
pub struct TransactionService;

impl TransactionService {
    #[tracing::instrument(skip(db))]
    pub async fn deposit_funds(
        &self,
        db: &PgPool,
        deposit_info: DepositRequest,
    ) -> Result<(), ServiceError> {
        let mut tx = db.begin().await?;

        // Get or create escrow account
        let escrow_service = EscrowService {};
        let escrow_account = escrow_service
            .get_or_create_escrow_users(&mut tx, &deposit_info.wallet_address)
            .await?;

        // Create transaction record
        tracing::info!("Creating Deposit Transaction");
        let now = Utc::now();
        let query = r#"
            INSERT INTO escrow_transactions
            (wallet_address, type, amount, currency, transaction_hash, status, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#;

        sqlx::query(query)
            .bind(&deposit_info.wallet_address)
            .bind(TransactionType::Deposit)
            .bind(BigDecimal::from(deposit_info.amount))
            .bind(deposit_info.currency)
            .bind(deposit_info.transaction_hash)
            .bind(TransactionStatus::Completed)
            .bind(deposit_info.notes)
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to create deposit transaction");
                ServiceError::DatabaseError(e)
            })?;

        tracing::info!("Updating escrow account balance");
        // Update escrow account balance
        let new_balance = escrow_account.balance + BigDecimal::from(deposit_info.amount);

        sqlx::query(
            r#"
            UPDATE escrow_users
            SET balance = $1, updated_at = $2
            WHERE wallet_address = $3
            "#,
        )
        .bind(new_balance)
        .bind(now)
        .bind(&deposit_info.wallet_address)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Failed to update escrow account balance");
            ServiceError::DatabaseError(e)
        })?;

        // Commit transaction
        tx.commit().await.map_err(|e| {
            tracing::error!(error = %e, "Failed to commit transaction");
            ServiceError::DatabaseError(e)
        })?;

        tracing::info!("Deposit transaction completed successfully");
        Ok(())
    }

    // fn
    pub async fn withdraw_funds(
        &self,
        db: &PgPool,
        withdrawal_request: WithdrawalRequest
    ) -> Result<(), ServiceError> {
        // Validate amount
        check_withdrawal_amount(&withdrawal_request.amount)?;

        let mut tx = db.begin().await?;

        // Get escrow user with lock for update
        let escrow_service = EscrowService {};
        let user = escrow_service.get_escrow_user(&mut tx, &withdrawal_request.wallet_address).await?;

        // Check sufficient balance
        check_withdrawal_amount_as_against_balance(&user.balance, &withdrawal_request.amount)?;

        // Calculate new balance
        let new_balance = user.balance - &withdrawal_request.amount;
        let transaction_hash = generate_transaction_hash();
        let current_date_time = Utc::now();

        // Update escrow account balance
        escrow_service.update_escrow_user_balance(&mut tx, &withdrawal_request.wallet_address, &new_balance).await?;

        // Create withdrawal transaction record
        self.create_transaction(&mut tx, &withdrawal_request.wallet_address, &withdrawal_request.amount, &withdrawal_request.currency, &withdrawal_request.notes).await?;

        // Commit transaction
        tx.commit().await.map_err(|e| {
            tracing::error!(error = %e, "Failed to commit withdrawal transaction");
            ServiceError::DatabaseError(e)
        })?;

        tracing::info!(
        wallet = %withdrawal_request.wallet_address,
        amount = %withdrawal_request.amount,
        "Withdrawal completed successfully"
    );
        Ok(())
    }

    async fn create_transaction(&self, tx: &mut Transaction<'_, Postgres>, wallet_address: &String, amount: &BigDecimal, currency: &String, notes: &Option<String>) -> Result<(), ServiceError> {
        let transaction_query = r#"
        INSERT INTO escrow_transactions (
            wallet_address,
            transaction_type,
            amount,
            currency,
            transaction_hash,
            transaction_status,
            notes,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
    "#;

        let current_date_time = Utc::now();
        let transaction_hash = generate_transaction_hash();

        sqlx::query(transaction_query)
            .bind(wallet_address)
            .bind(TransactionType::Withdrawal)
            .bind(amount)
            .bind(currency)
            .bind(&transaction_hash)
            .bind(TransactionStatus::Completed)
            .bind(notes)
            .bind(current_date_time)
            .bind(current_date_time)
            .execute(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to create withdrawal transaction");
                ServiceError::DatabaseError(e)
            })?;

        Ok(())
    }
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/deposit", post(deposit));
    Router::new().route("/withdraw", post(withdraw))
}

#[tracing::instrument(skip(state, payload))]
pub async fn deposit(state: State<AppState>, Json(payload): Json<DepositRequest>) -> StatusCode {
    let transaction_service = TransactionService {};
    if let Err(e) = transaction_service
        .deposit_funds(&state.db.pool, payload)
        .await
    {
        let (status_code, _) = From::from(e);
        return status_code;
    }

    tracing::info!("Deposit transaction created successfully");
    StatusCode::CREATED
}

#[tracing::instrument(skip(state, payload))]
pub async fn withdraw(state: State<AppState>, Json(payload): Json<WithdrawalRequest>) -> StatusCode {
    let transaction_service = TransactionService {};
    if let Err(e) = transaction_service
        .withdraw_funds(&state.db.pool, payload)
        .await
    {
        let (status_code, _) = From::from(e);
        return status_code;
    }

    StatusCode::CREATED
}
