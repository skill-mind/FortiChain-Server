use crate::services::utils::ServiceError;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;

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
    #[tracing::instrument(skip(db))]
    pub async fn get_or_create_escrow_users(
        &self,
        db: &PgPool,
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
            .fetch_optional(db)
            .await;
        if let Err(e) = existing_account {
            tracing::error!(error = %e, "Failed to fetch existing escrow account");
            return Err(ServiceError::DatabaseError(e));
        }
        let existing_account = existing_account.unwrap();
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
            .fetch_one(db)
            .await;
        if let Err(e) = new_account {
            tracing::error!(error = %e, "Failed to create new escrow account");
            return Err(ServiceError::DatabaseError(e));
        }
        let new_account = new_account.unwrap();
        tracing::info!(wallet = %new_account.wallet_address, "New escrow account created successfully");

        Ok(new_account)
    }
}
