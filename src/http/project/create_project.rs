use crate::{AppState, Error, Result, ResultExt, http::project::CreateProjectRequest};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "create_project", skip(state, payload))]
pub async fn create_project_handler(
    state: State<AppState>,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<(StatusCode, String)> {
    payload.validate()?;
    match (
        &payload.bounty_amount,
        &payload.bounty_currency,
        &payload.bounty_expiry_date,
    ) {
        (Some(_), Some(_), Some(_)) | (None, None, None) => {}
        _ => {
            return Err(Error::InvalidRequest(
                "Bounty fields must be all present or all null".to_string(),
            ));
        }
    }

    let mut tx = state.db.pool.begin().await?;

    let project_id = sqlx::query_scalar!(
        r#"
        INSERT INTO projects (
            owner_address, contract_address, name, description, contact_info,
            supporting_document_path, project_logo_path, repository_url,
            bounty_amount, bounty_currency, bounty_expiry_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id
        "#,
        payload.owner_address,
        payload.contract_address,
        payload.name,
        payload.description,
        payload.contact_info,
        payload.supporting_document_path,
        payload.project_logo_path,
        payload.repository_url,
        payload.bounty_amount,
        payload.bounty_currency,
        payload.bounty_expiry_date
    )
    .fetch_one(&mut *tx)
    .await
    .on_constraint("projects_contract_address_key", |_| Error::Conflict)?;

    if !payload.tags.is_empty() {
        let tag_ids: Vec<i32> = sqlx::query_scalar!(
            r#"
            WITH input_tags(tag_name) AS (
                SELECT unnest($1::text[])
            ),
            inserted_tags AS (
                INSERT INTO tags (name)
                SELECT tag_name FROM input_tags
                ON CONFLICT (name) DO NOTHING
                RETURNING id
            )
            SELECT id FROM inserted_tags
            UNION
            SELECT t.id FROM tags t
            JOIN input_tags it ON t.name = it.tag_name
            "#,
            &payload.tags as &[String]
        )
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .flatten()
        .collect();

        if !tag_ids.is_empty() {
            sqlx::query!(
                r#"
                INSERT INTO project_tags (project_id, tag_id)
                SELECT $1, unnest($2::int[])
                ON CONFLICT DO NOTHING
                "#,
                project_id,
                &tag_ids
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;
    Ok((
        StatusCode::CREATED,
        "Project created successfully".to_string(),
    ))
}
