use crate::{Error, Result, http::project::ProjectResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_project_by_id(pool: &PgPool, project_id: Uuid) -> Result<ProjectResponse> {
    let project = sqlx::query_as!(
        ProjectResponse,
        r#"
        SELECT
            id,
            name,
            owner_address,
            contract_address,
            description,
            is_verified,
            verification_date,
            repository_url,
            created_at
        FROM projects
        WHERE id = $1
        "#,
        project_id
    )
    .fetch_optional(pool)
    .await?;

    project.ok_or(Error::NotFound)
}
