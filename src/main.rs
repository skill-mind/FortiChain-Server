use fortichain_server::{Configuration, http};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let configuration = Configuration::new();
    http::serve(configuration)
        .await
        .expect("Failed to start server.");
}
