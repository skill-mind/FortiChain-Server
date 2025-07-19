use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    routing::post,
};
use sqlx::{Executor, Postgres};

use super::types::{RegisterValidatorParams, ValidatorProfile};
use crate::AppState;

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/validators", post(register_validator))
}

async fn register_validator(
    State(state): State<AppState>,
    Json(payload): Json<RegisterValidatorParams>,
) -> Result<(StatusCode, Json<ValidatorProfile>), StatusCode> {
    let mut tx = state
        .db
        .pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2) insert validator (UUID PK)
    let row = sqlx::query!(
        r#"
        INSERT INTO validators(wallet_address, name, bio)
        VALUES ($1, $2, $3)
        ON CONFLICT(wallet_address) DO NOTHING
        RETURNING id      AS "validator_id!: Uuid",
                  wallet_address,
                  name,
                  bio,
                  created_at,
                  updated_at
        "#,
        payload.wallet_address,
        payload.name,
        payload.bio,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::CONFLICT)?; // 409 on duplicate

    // helper: upsert into a text‑lookup table, returning its numeric ID
    async fn upsert_and_get_id(
        executor: impl Executor<'_, Database = Postgres>, // Generic executor
        table: &str,
        value: &str,
    ) -> Result<i32, sqlx::Error> {
        let sql = format!(
            "INSERT INTO {table} (value) VALUES ($1) ON CONFLICT (value) DO UPDATE SET value = EXCLUDED.value RETURNING id",
            table = table
        );
        sqlx::query_scalar(&sql)
            .bind(value)
            .fetch_one(executor)
            .await
    }

    // 3) languages (note: payload.programming_languages)
    for lang in &payload.programming_lang {
        let lang_id: i32 = upsert_and_get_id(&mut *tx, "programming_languages", lang)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        sqlx::query!(
            r#"
            INSERT INTO validator_programming_lang
              (validator_id, language_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            row.validator_id,
            lang_id,
        )
        .execute(&mut tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // 4) expertise areas (note: payload.expertise_areas)
    for area in &payload.expertise_area {
        let exp_id: i32 = upsert_and_get_id(&mut *tx, "expertise_areas", area)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        sqlx::query!(
            r#"
            INSERT INTO validator_expertise_area
              (validator_id, expertise_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            row.validator_id,
            exp_id,
        )
        .execute(&mut tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // 5) commit
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 6) build our JSON response
    let profile = ValidatorProfile {
        validator_id: row.validator_id,
        wallet_address: row.wallet_address,
        name: row.name,
        bio: row.bio,
        programming_languages: payload.programming_lang.clone(),
        expertise_areas: payload.expertise_area.clone(),
        created_at: row.created_at,
        updated_at: row.updated_at,
    };

    Ok((StatusCode::CREATED, Json(profile)))
}
