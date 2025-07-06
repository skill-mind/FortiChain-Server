use sqlx::{
    query_as,
    postgres::{PgPool},
};

#[derive(thiserror::Error, Debug)]
pub enum EscrowError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct EscrowUsers {
    wallet_address: String,
    balance: f64,
    created_at: f64,
    updated_at: f64
}


pub struct EscrowService {
    db: PgPool
}
