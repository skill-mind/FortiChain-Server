use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct OpenSupportTicketRequest {
    #[garde(ascii, length(min = 5, max = 100))]
    pub subject: String,
    #[garde(ascii, length(min = 10, max = 5000))]
    pub message: String,
    #[garde(ascii, length(bytes, equal = 66))]
    pub opened_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignSupportTicketRequest {
    pub ticket_id: Uuid,
    pub support_agent_wallet: String,
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

#[derive(Debug, Deserialize, Validate)]
pub struct ListTicketsQuery {
    #[garde(custom(validate_query_status))]
    pub status: Option<String>, // comma-separated
    #[garde(custom(validate_sort))]
    pub sort: Option<String>, // "asc" or "desc"
    #[garde(range(min = 1, max = 20))]
    pub limit: Option<i64>,
    #[garde(skip)]
    pub offset: Option<i64>,
}

pub fn validate_sort(sort: &Option<String>, _context: &()) -> garde::Result {
    if let Some(sort) = sort {
        if !["asc", "desc"].contains(&sort.as_str()) {
            return Err(garde::Error::new("Invalid sort specified"));
        }
    }
    Ok(())
}

pub fn validate_query_status(statuses: &Option<String>, _context: &()) -> garde::Result {
    if let Some(statuses) = statuses {
        let statuses = statuses.split(',').collect::<Vec<&str>>();
        for status in statuses {
            if ![
                "open",
                "assigned",
                "in_progress",
                "awaiting_user",
                "reopened",
                "resolved",
                "closed",
            ]
            .contains(&status)
            {
                return Err(garde::Error::new("Invalid status specified"));
            }
        }
    }
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct TicketHistoryParams {
    wallet: String,
    page: Option<u32>,
    limit: Option<u32>,
}
