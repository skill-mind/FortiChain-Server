use sqlx::{Pool, Postgres, Error};
use crate::http::types::{NewReportRequest, ResearchReport};

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn create_report(
        &self,
        new: NewReportRequest,
    ) -> Result<ResearchReport, Error> {
        let rec = sqlx::query_as::<_, ResearchReport>(
            r#"
                INSERT INTO research_report
                  (title, project_id, content, reported_by)
                VALUES ($1, $2, $3, $4)
                RETURNING
                  id,
                  title,
                  project_id,
                  content,
                  reported_by,
                  created_at
            "#,
        )
        .bind(&new.title)
        .bind(&new.project_id)
        .bind(&new.content)
        .bind(&new.reported_by)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}