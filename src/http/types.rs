use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;

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