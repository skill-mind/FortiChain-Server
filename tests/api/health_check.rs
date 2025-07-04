use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use crate::helpers::TestApp;
use fortichain_server::db::DbPool;
use sqlx::Row;

#[tokio::test]
async fn test_health_check_ok() {
    let app = TestApp::new().await;
    let db = &app.db;

    // Create a test table
    match db.pool() {
        DbPool::Sqlite(pool) => {
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS test_items (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
            )
            .execute(pool)
            .await
            .expect("Failed to create test table");

            // Insert an item
            let name = "Test Item";
            let row = sqlx::query("INSERT INTO test_items (name) VALUES (?) RETURNING id")
                .bind(name)
                .fetch_one(pool)
                .await
                .expect("Failed to insert item");
            let id: i64 = row.get("id");

            // Retrieve the item
            let retrieved = sqlx::query("SELECT name FROM test_items WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .expect("Failed to retrieve item");
            let retrieved_name: String = retrieved.get("name");
            assert_eq!(
                retrieved_name, name,
                "Retrieved name should match inserted name"
            );

            // Clean up
            sqlx::query("DROP TABLE test_items")
                .execute(pool)
                .await
                .expect("Failed to clean up");
        }
        DbPool::Postgres(pool) => {
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS test_items (id SERIAL PRIMARY KEY, name TEXT NOT NULL)",
            )
            .execute(pool)
            .await
            .expect("Failed to create test table");

            // Insert an item
            let name = "Test Item";
            let row = sqlx::query("INSERT INTO test_items (name) VALUES ($1) RETURNING id")
                .bind(name)
                .fetch_one(pool)
                .await
                .expect("Failed to insert item");
            let id: i32 = row.get("id");

            // Retrieve the item
            let retrieved = sqlx::query("SELECT name FROM test_items WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await
                .expect("Failed to retrieve item");
            let retrieved_name: String = retrieved.get("name");
            assert_eq!(
                retrieved_name, name,
                "Retrieved name should match inserted name"
            );

            // Clean up
            sqlx::query("DROP TABLE test_items")
                .execute(pool)
                .await
                .expect("Failed to clean up");
        }
    }

    let req = Request::get("/health_check").body(Body::empty()).unwrap();
    let res = app.request(req).await;

    assert_eq!(res.status(), StatusCode::OK);
}
