use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    BountyAllocation
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed
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
    user_wallet: String,
    project_id: String,
    amount: u128,
    currency: String,
    notes: Option<String>,
    transaction_hash: String
}

pub struct TransactionService {
    db: PgPool
}

impl TransactionService {
    pub fn new(db: PgPool) -> Self{
        Self {db}
    } 
    pub fn deposit_funds(&self, deposit_info: DepositRequest) -> Transaction {

    }   
}

