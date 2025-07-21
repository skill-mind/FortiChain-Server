use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
<<<<<<< HEAD

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
=======
>>>>>>> 5b5c072 (CX)

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSupportTicketRequest {
    pub subject: String,
    pub message: String,
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

#[derive(Debug, Deserialize)]
pub struct ListTicketsQuery {
    pub status: Option<String>, // comma-separated
    pub sort: Option<String>,   // "asc" or "desc"
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

<<<<<<< HEAD
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
=======
// --- NEW TYPES FOR REPORT SUBMISSION ---

/// Payload for creating a new research report
#[derive(Debug, Deserialize)]
pub struct NewReportRequest {
    /// Title of the report
    pub title: String,
    /// Body/content of the report
    pub body: String,
    /// Associated project UUID
    pub project_id: Uuid,
    /// Researcher (reporter) UUID
    pub reported_by: Uuid,
}

/// Shape of a report returned after creation
#[derive(Debug, Serialize)]
pub struct ReportResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub project_id: Uuid,
    pub researcher_id: Uuid,
    pub created_at: String,
>>>>>>> 5b5c072 (CX)
}
