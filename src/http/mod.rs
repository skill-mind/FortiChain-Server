use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod create_project;
pub mod escrow;
pub mod health_check;
pub mod helpers;
pub mod projects;
pub mod research_report; // ✅ This now points to the proper file
pub mod support_tickets;
pub mod transaction;
pub mod types; // ✅ This now points to the proper file

use std::sync::Arc;

use crate::{Config, Configuration, db::Db};
use anyhow::Context;
use axum::Router;
use tokio::{net::TcpListener, signal};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub configuration: Config,
}

/// Compose all API routes under the root
pub fn api_router(app_state: AppState) -> Router {
    Router::new()
        .merge(health_check::router())
        .merge(research_report::router())
        .merge(support_tickets::router())
        .merge(transaction::router())
        .merge(projects::router())
        .with_state(app_state)
}

pub async fn serve(configuration: Arc<Configuration>, db: Db) -> anyhow::Result<()> {
    let addr = configuration.listen_address;
    let app_state = AppState { configuration, db };
    let app = api_router(app_state);

    tracing::info!("Listening for requests on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("error running HTTP server.")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to configure ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to configure SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}


// ===== create_project.rs needs this =====
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
    pub bounty_expiry_date: Option<DateTime<chrono::Utc>>,
}

// ===== escrow.rs needs this =====
#[derive(Debug, Deserialize)]
pub struct AllocateBountyRequest {
    pub wallet_address: String,
    pub project_contract_address: String,
    pub amount: BigDecimal,
    pub currency: String,
    pub bounty_expiry_date: Option<DateTime<chrono::Utc>>,
}

// ===== support_tickets.rs needs these =====
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
    pub resolved_by: String,
    pub resolution_response: String,
}

#[derive(Debug, Deserialize)]
pub struct ListTicketsQuery {
    pub status: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<u32>,
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
