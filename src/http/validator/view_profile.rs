use crate::{AppState, Error, Result};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
// ...existing code...

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ViewValidatorProfileRequest {
    #[garde(custom(validate_starknet_address))]
    pub wallet_address: String,
}

#[derive(Debug, Serialize)]
pub struct ValidatorProfileResponse {
    pub government_name: String,
    pub email_address: String,
    pub years_of_experience: i16,
    pub verification: String,
    pub programming_languages: Vec<String>,
    pub expertise: Vec<String>,
}

#[tracing::instrument(name = "View Validator Profile", skip(state, request))]
pub async fn view_validator_profile(
    State(state): State<AppState>,
    Json(request): Json<ViewValidatorProfileRequest>,
) -> Result<(StatusCode, Json<ValidatorProfileResponse>)> {
    request.validate()?;

    let profile = get_validator_profile(&state.db.pool, &request.wallet_address).await?;
    if let Some(profile) = profile {
        Ok((StatusCode::OK, Json(profile)))
    } else {
        Err(Error::NotFound)
    }
}

async fn get_validator_profile(
    pool: &PgPool,
    wallet_address: &str,
) -> Result<Option<ValidatorProfileResponse>> {
    #[derive(sqlx::FromRow)]
    struct ValidatorProfileRow {
        government_name: String,
        email_address: String,
        years_of_experience: i16,
        verification: String,
        programming_languages: Option<Vec<String>>,
        expertise: Option<Vec<String>>,
    }

    let row = sqlx::query_as::<_, ValidatorProfileRow>(
        r#"
        SELECT
            vp.government_name,
            vp.email_address,
            vp.years_of_experience,
            vp.verification,
            ARRAY_AGG(DISTINCT pl.name) AS programming_languages,
            ARRAY_AGG(DISTINCT ex.name) AS expertise
        FROM
            validator_profiles vp
        LEFT JOIN
            validator_programming_languages vpl ON vp.id = vpl.validator_id
        LEFT JOIN
            programming_languages pl ON vpl.language_id = pl.id
        LEFT JOIN
            validator_expertise vex ON vp.id = vex.validator_id
        LEFT JOIN
            expertise ex ON vex.expertise_id = ex.id
        WHERE
            vp.wallet_address = $1
        GROUP BY
            vp.id
        "#
    )
    .bind(wallet_address)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some(ValidatorProfileResponse {
            government_name: row.government_name,
            email_address: row.email_address,
            years_of_experience: row.years_of_experience,
            verification: row.verification,
            programming_languages: row.programming_languages.unwrap_or_default(),
            expertise: row.expertise.unwrap_or_default(),
        }))
    } else {
        Ok(None)
    }
}

pub fn validate_starknet_address(address: &str, _context: &()) -> garde::Result {
    if address.starts_with("0x")
        && address.len() == 66
        && address.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(garde::Error::new("Invalid Starknet address"))
    }
}
