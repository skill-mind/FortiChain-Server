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

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterValidatorParams {
    pub wallet_address: String,
    pub name: String,
    pub bio: String,
    pub programming_lang: Vec<String>,
    pub expertise_area: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidatorProfile {
    pub validator_id: Uuid,
    pub wallet_address: String,
    pub name: String,
    pub bio: Option<String>,
    pub programming_languages: Vec<String>,
    pub expertise_areas: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
