use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod create_project;
pub mod escrow;
pub mod health_check;
pub mod helpers;
pub mod projects;
pub mod research_report;
pub mod support_tickets;
pub mod transaction;
pub mod types;

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
