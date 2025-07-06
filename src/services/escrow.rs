use sqlx::{postgres::PgPool, query_as};

use crate::services::utils::ServiceError;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct EscrowUsers {
    pub wallet_address: String,
    pub balance: u128,
    pub created_at: f64,
    pub updated_at: f64,
}

pub struct EscrowService {
    pub db: PgPool,
}

impl EscrowService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // Create or get existing escrow account for user
    pub async fn get_or_create_escrow_users(
        &self,
        user_wallet: String,
    ) -> Result<EscrowUsers, ServiceError> {
        // First, try to get existing account
        let existing_account = query_as::<_, EscrowUsers>(
            "
            SELECT wallet_address, balance, created_at, updated_at
            FROM escrow_users
            WHERE wallet_address = $1
            ",
        )
        .bind(user_wallet)
        .fetch_optional(&self.db)
        .await?;

        if let Some(account) = existing_account {
            return Ok(account);
        }

        // Create new account if it doesn't exist
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

        let new_account = sqlx::query_as!(
            EscrowUsers,
            r#"
            INSERT INTO escrow_users (wallet_address, balance, created_at, updated_at)
            VALUES ($1, $2, $3, $4,)
            RETURNING wallet_address, balance, created_at, updated_at
            "#,
            user_wallet.clone(),
            0,
            now,
            now
        )
        .fetch_one(&self.db)
        .await?;

        Ok(new_account)
    }
}
