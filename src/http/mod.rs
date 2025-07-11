use std::sync::Arc;

use crate::Config;
use crate::{Configuration, db::Db};
use anyhow::Context;
use axum::Router;
use tokio::{net::TcpListener, signal};

mod create_project;
mod health_check;
mod projects;
mod support_tickets;
mod types;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub configuration: Config,
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

pub fn api_router(app_state: AppState) -> Router {
    Router::new()
        .merge(health_check::router())
        .merge(projects::router())
        .merge(support_tickets::router())
        .merge(create_project::router())
        .with_state(app_state)
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

    tokio::select! {_ = ctrl_c => {}, _ = terminate => {},}
}
