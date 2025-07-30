use crate::{
    AppState, Error, Result,
    http::validator::{DeleteValidatorProfileRequest, DeleteValidatorProfileResponse},
};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Delete Validator Profile", skip(state, request))]
pub async fn delete_validator_profile(
    State(state): State<AppState>,
    Json(request): Json<DeleteValidatorProfileRequest>,
) -> Result<(StatusCode, Json<DeleteValidatorProfileResponse>)> {
    request.validate()?;

    tracing::info!(
        wallet_address = %request.wallet_address,
        "Attempting to delete validator profile"
    );

    // First, verify the validator profile exists
    let validator =
        get_validator_by_wallet_address(&state.db.pool, &request.wallet_address).await?;

    if validator.is_none() {
        tracing::warn!(
            wallet_address = %request.wallet_address,
            "Validator profile not found"
        );
        return Err(Error::NotFound);
    }

    let validator = validator.unwrap();
    let deletion_time = chrono::Utc::now();

    // Perform the deletion with all related data cleanup
    delete_validator_and_related_data(&state.db.pool, &validator.id, &request.wallet_address)
        .await?;

    tracing::info!(
        validator_id = %validator.id,
        wallet_address = %request.wallet_address,
        "Successfully deleted validator profile and all related data"
    );

    Ok((
        StatusCode::OK,
        Json(DeleteValidatorProfileResponse {
            message: "Validator profile successfully deleted".to_string(),
            validator_id: validator.id,
            deleted_at: deletion_time,
        }),
    ))
}

#[derive(Debug, sqlx::FromRow)]
struct ValidatorInfo {
    id: Uuid,
}

async fn get_validator_by_wallet_address(
    pool: &PgPool,
    wallet_address: &str,
) -> Result<Option<ValidatorInfo>> {
    let validator = sqlx::query_as::<_, ValidatorInfo>(
        r#"
        SELECT id
        FROM validator_profiles
        WHERE wallet_address = $1
        "#,
    )
    .bind(wallet_address)
    .fetch_optional(pool)
    .await?;

    Ok(validator)
}

async fn delete_validator_and_related_data(
    pool: &PgPool,
    validator_id: &Uuid,
    wallet_address: &str,
) -> Result<()> {
    let mut tx = pool.begin().await?;

    // Delete from validator_expertise (many-to-many relationship)
    let expertise_deleted = sqlx::query("DELETE FROM validator_expertise WHERE validator_id = $1")
        .bind(validator_id)
        .execute(&mut *tx)
        .await?;

    tracing::debug!(
        validator_id = %validator_id,
        expertise_rows_deleted = expertise_deleted.rows_affected(),
        "Deleted validator expertise relationships"
    );

    // Delete from validator_programming_languages (many-to-many relationship)
    let languages_deleted =
        sqlx::query("DELETE FROM validator_programming_languages WHERE validator_id = $1")
            .bind(validator_id)
            .execute(&mut *tx)
            .await?;

    tracing::debug!(
        validator_id = %validator_id,
        language_rows_deleted = languages_deleted.rows_affected(),
        "Deleted validator programming language relationships"
    );

    // Finally, delete the validator profile itself
    let profile_deleted = sqlx::query("DELETE FROM validator_profiles WHERE id = $1")
        .bind(validator_id)
        .execute(&mut *tx)
        .await?;

    if profile_deleted.rows_affected() == 0 {
        tracing::error!(
            validator_id = %validator_id,
            "Failed to delete validator profile - no rows affected"
        );
        return Err(Error::InternalServerError(anyhow::anyhow!(
            "Failed to delete validator profile"
        )));
    }

    // Also remove from escrow_users if they exist there
    let escrow_deleted = sqlx::query("DELETE FROM escrow_users WHERE wallet_address = $1")
        .bind(wallet_address)
        .execute(&mut *tx)
        .await?;

    tracing::debug!(
        validator_id = %validator_id,
        escrow_deleted = escrow_deleted.rows_affected(),
        "Deleted validator from escrow_users if present"
    );

    tx.commit().await?;

    tracing::info!(
        validator_id = %validator_id,
        "Successfully committed validator profile deletion transaction"
    );

    Ok(())
}
