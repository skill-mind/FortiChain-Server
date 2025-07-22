use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub owner_address: String,
    pub contract_address: String,
    pub name: String,
    pub description: String,
    pub contact_info: String,
    pub supporting_document_path: Option<String>,
    pub project_logo_path: Option<String>,
    pub repository_url: Option<String>,
    pub tags: Vec<String>,
    pub bounty_amount: Option<BigDecimal>, // numeric(20,2)
    pub bounty_currency: Option<String>,
    pub bounty_expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct OpenSupportTicketRequest {
    #[garde(ascii, length(min = 5, max = 100))]
    pub subject: String,
    #[garde(ascii, length(min = 10, max = 5000))]
    pub message: String,
    #[garde(ascii, length(bytes, equal = 66))]
    pub opened_by: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupportTicket {
    pub id: String,
    pub subject: String,
    pub message: String,
    pub document_path: Option<String>,
    pub opened_by: String,
    pub status: String,
    pub assigned_to: Option<String>,
    pub response_subject: String,
    pub resolution_response: Option<String>,
    pub resolved: bool,
    pub created_at: String,
    pub resolved_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResolveSupportTicketRequest {
    #[garde(skip)]
    pub ticket_id: Uuid,
    #[garde(ascii, length(min = 10, max = 5000))]
    pub resolution_response: String,
    #[garde(ascii, length(bytes, equal = 66))]
    pub resolved_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignSupportTicketRequest {
    pub ticket_id: Uuid,
    pub support_agent_wallet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClosedProjectRequest {
    pub project_id: String,
    pub owner_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllocateBountyRequest {
    pub wallet_address: String,
    pub project_contract_address: String,
    pub amount: BigDecimal,
    pub currency: String,
    pub bounty_expiry_date: Option<DateTime<Utc>>, // ISO8601 string
}
