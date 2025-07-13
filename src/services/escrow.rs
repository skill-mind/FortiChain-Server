use sqlx::postgres::PgPool;

use crate::services::utils::ServiceError;
use time::OffsetDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct EscrowUsers {
    pub wallet_address: String,
    pub balance: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub struct EscrowService;

impl EscrowService {
    // Create or get existing escrow account for user
    pub async fn get_or_create_escrow_users(
        &self,
        db: &PgPool,
        user_wallet: String,
    ) -> Result<EscrowUsers, ServiceError> {
        // First, try to get existing account
        let query = r#"
            SELECT wallet_address, balance, created_at, updated_at
            FROM escrow_users
            WHERE wallet_address = $1
        "#;
        let existing_account = sqlx::query_as::<_, EscrowUsers>(&query)
            .bind(user_wallet.clone())
            .fetch_optional(db)
            .await?;

        if let Some(account) = existing_account {
            return Ok(account);
        }

        // Create new account if it doesn't exist
        let now = OffsetDateTime::now_utc();
        let create_account_query = r#"
            INSERT INTO escrow_users (wallet_address, balance, created_at, updated_at)
            VALUES ($1, $2, $3, $4,)
            RETURNING wallet_address, balance, created_at, updated_at
            "#;
        let new_account = sqlx::query_as::<_, EscrowUsers>(create_account_query)
            .bind(user_wallet)
            .bind(0)
            .bind(now)
            .bind(now)
            .fetch_one(db)
            .await?;

        Ok(new_account)
    }
}
