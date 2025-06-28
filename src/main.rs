use fortichain_server::{Configuration, http, telemetry};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize tracing
    telemetry::setup_tracing();

    tracing::debug!("Initializing configuration");
    let config = Configuration::new();

    tracing::info!("Starting server on {}", config.listen_address);

    http::serve(config).await.expect("Failed to start server.");
}
