use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;

#[derive(Debug, Deserialize)]
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
    pub bounty_amount: Option<BigDecimal>,
    pub bounty_currency: Option<String>,
    pub bounty_expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AllocateBountyRequest {
    pub wallet_address: String,
    pub project_contract_address: String,
    pub amount: BigDecimal,
    pub currency: String,
    pub bounty_expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct OpenSupportTicketRequest {
    pub subject: String,
    pub message: String,
    pub opened_by: String,
}

#[derive(Debug, Deserialize)]
pub struct AssignSupportTicketRequest {
    pub ticket_id: Uuid,
    pub support_agent_wallet: String,
}

#[derive(Debug, Deserialize)]
pub struct ResolveSupportTicketRequest {
    pub ticket_id: String,
    pub resolution_response: String,
    pub resolved_by: String,
}

#[derive(Debug, Deserialize)]
pub struct ListTicketsQuery {
    pub status: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SupportTicket {
    pub id: String,
    pub subject: String,
    pub message: String,
    pub document_path: Option<String>,
    pub opened_by: String,
    pub status: String,
    pub assigned_to: Option<String>,
    pub response_subject: Option<String>,
    pub resolution_response: Option<String>,
    pub resolved: Option<bool>,
    pub created_at: String,
    pub resolved_at: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReportRequest {
    pub title: String,
    pub content: String,
    pub project_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct NewReportRequest {
    pub title: String,
    pub content: String,
    pub project_id: Option<Uuid>,
    pub reported_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ResearchReport {
    pub id: Uuid,
    pub title: String,
    pub project_id: Option<Uuid>,
    pub content: String,
    pub reported_by: Uuid,
    pub created_at: DateTime<Utc>,
}
