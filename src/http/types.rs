use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AllocateBountyRequest {
    pub wallet_address: String,
    pub project_contract_address: String,
    pub amount: BigDecimal,
    pub currency: String,
    pub bounty_expiry_date: Option<DateTime<Utc>>, // ISO8601 string
}
