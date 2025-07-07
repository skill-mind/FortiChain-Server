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

    // Create a new research report
    pub async fn create_report(&self, new: NewResearchReport) -> Result<ResearchReport, Error> {
        let rec = sqlx::query_as!(
            ResearchReport,
            r#"
            INSERT INTO research_report (title, project_id, body, reported_by)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id, title, project_id, body, reported_by,
                validated_by, status::text, severity::text,
                allocated_reward, reason::text, validator_notes,
                researcher_response, created_at, updated_at
            "#,
            new.title,
            new.project_id,
            new.body,
            new.reported_by
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }
