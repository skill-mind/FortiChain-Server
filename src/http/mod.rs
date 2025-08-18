use crate::telemetry::trace_layer;
use crate::{
    Config, cors_layer, normalize_path_layer, propagate_request_id_layer, request_id_layer,
    timeout_layer,
};
use crate::{Configuration, db::Db};
use anyhow::Context;
use axum::Router;
use std::sync::Arc;
use tokio::{net::TcpListener, signal};

pub use crate::error::{Error, ResultExt};
pub type Result<T, E = Error> = std::result::Result<T, E>;

mod escrow;
mod health_check;
pub mod newsletter;
mod project;
mod report;
mod support_ticket;
mod transaction;
mod types;
mod validator;

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
    let trace_layer = trace_layer();
    let request_id_layer = request_id_layer();
    let propagate_request_id_layer = propagate_request_id_layer();
    let cors_layer = cors_layer();
    let timeout_layer = timeout_layer();
    let normalize_path_layer = normalize_path_layer();

    Router::new()
        .merge(health_check::router())
        .merge(project::router())
        .merge(transaction::router())
        .merge(support_ticket::router())
        .merge(escrow::router())
        .merge(newsletter::router())
        .merge(validator::router())
        .merge(report::router())
        .layer(trace_layer)
        .layer(request_id_layer)
        .layer(propagate_request_id_layer)
        .layer(cors_layer)
        .layer(timeout_layer)
        .layer(normalize_path_layer)
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
