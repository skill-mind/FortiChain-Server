use anyhow::Result;
use sqlx::{
    Error,
    postgres::{PgPool, PgPoolOptions},
};

// Wrapper type to hold the DB pool.
#[derive(Clone)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {
    // Initialize a DB connection and return the Pool.
    pub async fn new(db_str: &str, max_pool_size: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_pool_size)
            .connect(db_str)
            .await?;
        Ok(Db { pool })
    }

    // executes a simple SELECT 1 query to verify database connectivity
    pub async fn ping_db(pool: &PgPool) -> Result<(), Error> {
        sqlx::query("SELECT 1").execute(pool).await?;
        Ok(())
    }

    // Run DB migrations
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
