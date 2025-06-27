use crate::Configuration;
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
    pub async fn new(config: &Configuration) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_db_connections)
            .connect(&config.database_url)
            .await?;
        Ok(Db { pool })
    }

    // executes a simple SELECT 1 query to verify database connectivity
    pub async fn ping_db(pool: &PgPool) -> Result<(), Error> {
        sqlx::query("SELECT 1").execute(pool).await?;
        Ok(())
    }
}
