use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::AppState;

use super::types::{RegisterValidatorParams, ValidatorProfile};

pub(crate) fn router() -> Router<AppState> {
    Router::new().route(
        "/validators",
        post(register_validator),
    )
}

async fn register_validator(
    State(db): State<PgPool>,
    Json(payload): Json<RegisterValidatorParams>,
) -> Result<(StatusCode, Json<ValidatorProfile>), StatusCode> {
    // wrap in a transaction so we rollback on any failure
    let mut tx = db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 1) insert validator (enforces unique wallet_address)
    let validator = sqlx::query!(
        r#"
        INSERT INTO validators(wallet_address, name, bio)
        VALUES ($1, $2, $3)
        ON CONFLICT(wallet_address) DO NOTHING
        RETURNING id, wallet_address, name, bio, created_at, updated_at
        "#,
        payload.wallet_address,
        payload.name,
        payload.bio,
    )
    .fetch_optional(&mut tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = validator
        .ok_or(StatusCode::CONFLICT)?; // already exists → 409

    let validator_id: Uuid = row.id;

    // Helper: upsert into a master table and return its UUID
    async fn upsert_and_get_id(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        table: &str,
        value: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let q = format!(
            "INSERT INTO {table}(name) VALUES ($1)
             ON CONFLICT(name) DO UPDATE SET name = EXCLUDED.name
             RETURNING id"
        );
        let rec = sqlx::query_scalar(&q)
            .bind(value)
            .fetch_one(&mut *tx)
            .await?;
        Ok(rec)
    }

    // 2) languages
    for lang in &payload.programming_lang {
        let lang_id = upsert_and_get_id(&mut tx, "programming_languages", lang).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        sqlx::query!(
            r#"
            INSERT INTO validator_programming_languages
            (validator_id, language_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            validator_id,
            lang_id,
        )
        .execute(&mut tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // 3) expertise areas
    for area in &payload.expertise_area {
        let exp_id = upsert_and_get_id(&mut tx, "expertise_areas", area).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        sqlx::query!(
            r#"
            INSERT INTO validator_expertise_areas
            (validator_id, expertise_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            validator_id,
            exp_id,
        )
        .execute(&mut tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // commit everything
    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // build response
    let profile = ValidatorProfile {
        validator_id,
        wallet_address: row.wallet_address,
        name: row.name,
        bio: row.bio,
        programming_languages: payload.programming_lang,
        expertise_areas: payload.expertise_area,
        created_at: row.created_at,
        updated_at: row.updated_at,
    };

    Ok((StatusCode::CREATED, Json(profile)))
}
