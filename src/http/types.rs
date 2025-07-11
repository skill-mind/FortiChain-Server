use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSupportTicketRequest {
    pub subject: String,
    pub message: String,
    pub opened_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveSupportTicketRequest {
    pub ticket_id: String,
    pub resolution_response: String,
    pub resolved_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignSupportTicketRequest {
    pub ticket_id: Uuid,
    pub support_agent_wallet: String,
}
