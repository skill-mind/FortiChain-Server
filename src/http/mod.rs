use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;

use crate::Configuration;

mod health_check;

#[derive(Clone)]
pub struct AppState {
    pub configuration: Arc<Configuration>,
}

pub async fn serve(configuration: Configuration) -> anyhow::Result<()> {
    let addr = configuration.listen_address;
    let app_state = AppState {
        configuration: Arc::new(configuration),
    };

    let app = api_router(app_state);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .await
        .context("error running HTTP server.")
}

pub fn api_router(app_state: AppState) -> Router {
    Router::new()
        .merge(health_check::router())
        .with_state(app_state)
}
