use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RejectReportRequest {
    #[garde(skip)]
    pub report_id: Uuid,
    #[garde(custom(validate_rejection_reason))]
    pub reason: String,
    #[garde(ascii, length(max = 1000))]
    pub validator_notes: Option<String>,
    #[garde(custom(validate_starknet_address))]
    pub validated_by: String,
}

#[derive(Debug, Serialize)]
pub struct RejectReportResponse {
    pub message: String,
    pub report_id: Uuid,
    pub status: String,
    pub reason: String,
    pub validator_notes: Option<String>,
    pub validated_by: String,
    pub rejected_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)] // Fields are part of database schema and will be used in future functionality
pub struct Report {
    pub id: Uuid,
    pub title: String,
    pub project_id: Uuid,
    pub body: String,
    pub reported_by: String,
    pub validated_by: Option<String>,
    pub status: String,           // Cast from enum to string in query
    pub severity: Option<String>, // Cast from enum to string in query
    pub allocated_reward: Option<sqlx::types::BigDecimal>, // Use BigDecimal for numeric fields
    pub reason: Option<String>,   // Cast from enum to string in query
    pub validator_notes: Option<String>,
    pub researcher_response: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub fn validate_rejection_reason(reason: &str, _context: &()) -> garde::Result {
    let valid_reasons = [
        "duplicate_report",
        "incomplete_information",
        "already_known",
        "out_of_scope",
    ];

    if valid_reasons.contains(&reason) {
        Ok(())
    } else {
        Err(garde::Error::new("Invalid rejection reason"))
    }
}

pub fn validate_starknet_address(address: &str, _context: &()) -> garde::Result {
    if address.starts_with("0x")
        && address.len() == 66
        && address.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(garde::Error::new("Invalid Starknet address"))
    }
}
