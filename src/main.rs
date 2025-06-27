use fortichain_server::{Configuration, db::Db, http};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let configuration = Configuration::new();
    let db = Db::new(&configuration)
        .await
        .expect("Failed to initialize DB");
    http::serve(configuration, db)
        .await
        .expect("Failed to start server.");
}
