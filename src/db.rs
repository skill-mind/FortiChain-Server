use anyhow::Result;
use sqlx::{
    Error,
    postgres::{PgPool, PgPoolOptions},
    sqlite::{SqlitePool, SqlitePoolOptions},
    migrate::MigrateDatabase,
};

use crate::config::DatabaseType;

// Wrapper type to hold the DB pool.
#[derive(Clone)]
pub struct Db {
    pub pool: DbPool,
    pub db_type: DatabaseType,
}

// Enum to hold different types of database pools
#[derive(Clone)]
pub enum DbPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

impl Db {
    // Initialize a DB connection and return the Pool.
    pub async fn new(db_str: &str, max_pool_size: u32, db_type: DatabaseType) -> Result<Self> {
        let pool = match db_type {
            DatabaseType::Postgres => {
                let pool = PgPoolOptions::new()
                    .max_connections(max_pool_size)
                    .connect(db_str)
                    .await?;
                DbPool::Postgres(pool)
            }
            DatabaseType::Sqlite => {
                if !sqlx::Sqlite::database_exists(db_str).await.unwrap_or(false) {
                    sqlx::Sqlite::create_database(db_str).await?;
                }
                let pool = SqlitePoolOptions::new()
                    .max_connections(max_pool_size)
                    .connect(db_str)
                    .await?;
                DbPool::Sqlite(pool)
            }
        };

        Ok(Db { pool, db_type })
    }

    // executes a simple SELECT 1 query to verify database connectivity
    pub async fn ping_db(&self) -> Result<(), Error> {
        match &self.pool {
            DbPool::Postgres(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
            DbPool::Sqlite(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
        }
        Ok(())
    }

    // Run DB migrations
    pub async fn migrate(&self) -> Result<()> {
        match &self.pool {
            DbPool::Postgres(pool) => {
                sqlx::migrate!("./migrations/postgres").run(pool).await?;
            }
            DbPool::Sqlite(pool) => {
                sqlx::migrate!("./migrations/sqlite").run(pool).await?;
            }
        }
        Ok(())
    }

    // Get the underlying pool for query execution
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
}
