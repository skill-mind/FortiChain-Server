use crate::{AppState, Error, Result, http::project::ClosedProjectRequest};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;

#[tracing::instrument(name = "close_project", skip(state, payload))]
pub async fn close_project_handler(
    state: State<AppState>,
    Json(payload): Json<ClosedProjectRequest>,
) -> Result<axum::http::StatusCode> {
    payload.validate()?;
    let mut tx = state.db.pool.begin().await?;

    let result = sqlx::query!(
        r#"
            WITH project AS (
                SELECT
                    p.owner_address,
                    p.bounty_amount,
                    p.closed_at,
                    EXISTS (SELECT 1 FROM escrow_users u WHERE u.wallet_address = $1) AS user_exists
                FROM projects p
                WHERE p.id = $2
                FOR UPDATE
            ),
            escrow_update AS (
                UPDATE escrow_users
                SET balance = COALESCE(balance, 0) + (
                    SELECT COALESCE(bounty_amount, 0)
                    FROM project
                    WHERE user_exists
                        AND owner_address = $1
                        AND closed_at IS NULL
                        AND COALESCE(bounty_amount, 0) > 0
                )
                WHERE wallet_address = $1
                AND (SELECT user_exists FROM project)
                AND (SELECT COALESCE(bounty_amount, 0) > 0 FROM project)
                AND (SELECT closed_at IS NULL FROM project)
                RETURNING 1
            ),
            project_update AS (
                UPDATE projects
                SET closed_at = now()
                WHERE id = $2
                AND EXISTS (SELECT 1 FROM project
                            WHERE owner_address = $1
                            AND user_exists
                            AND closed_at IS NULL)
                RETURNING 1
            )
            SELECT
                (SELECT owner_address FROM project) AS owner_address,
                (SELECT closed_at FROM project) AS closed_at,
                (SELECT user_exists FROM project) AS user_exists,
                (SELECT COUNT(*) FROM project_update) AS projects_updated
            "#,
        payload.owner_address,
        payload.project_id
    )
    .fetch_optional(&mut *tx)
    .await?;

    let Some(result) = result else {
        return Err(Error::NotFound);
    };

    match (result.owner_address, result.user_exists, result.closed_at) {
        (Some(owner_address), _, _) if owner_address != payload.owner_address => {
            return Err(Error::Unauthorized);
        }
        (_, Some(false), _) | (_, _, Some(_)) => return Err(Error::Forbidden),
        _ => {}
    };

    if result.projects_updated.unwrap_or(0) == 0 {
        return Err(Error::NotFound);
    }

    tx.commit().await?;
    Ok(StatusCode::OK)
}
