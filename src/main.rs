use fortichain_server::{Configuration, http, init_tracing};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

 // Initialize tracing
    init_tracing();

    let configuration = Configuration::new();
    http::serve(configuration)
        .await
        .expect("Failed to start server.");
}
