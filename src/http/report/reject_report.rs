use crate::{
    AppState, Error, Result,
    http::report::{RejectReportRequest, RejectReportResponse, Report},
};
use axum::{Json, extract::State, http::StatusCode};
use garde::Validate;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Reject Report", skip(state, request))]
pub async fn reject_report(
    State(state): State<AppState>,
    Json(request): Json<RejectReportRequest>,
) -> Result<(StatusCode, Json<RejectReportResponse>)> {
    request.validate()?;

    tracing::info!(
        report_id = %request.report_id,
        validated_by = %request.validated_by,
        "Attempting to reject report"
    );

    // First, verify the report exists and is in a valid state for rejection
    let report = get_report_by_id(&state.db.pool, &request.report_id).await?;

    if report.is_none() {
        tracing::warn!(
            report_id = %request.report_id,
            "Report not found"
        );
        return Err(Error::NotFound);
    }

    let report = report.unwrap();

    // Check if the report is already rejected or in a final state
    if report.status == "rejected" || report.status == "closed" {
        tracing::warn!(
            report_id = %request.report_id,
            status = %report.status,
            "Cannot reject report that is already in final state"
        );
        return Err(Error::Conflict);
    }

    // Verify the validator is authorized to reject this report
    if let Some(assigned_validator) = &report.validated_by {
        if assigned_validator != &request.validated_by {
            tracing::warn!(
                report_id = %request.report_id,
                assigned_validator = %assigned_validator,
                request_validator = %request.validated_by,
                "Validator not authorized to reject this report"
            );
            return Err(Error::Forbidden);
        }
    }
    // If no validator is assigned, any validator can reject (based on business logic)

    // Perform the rejection
    let rejection_time = chrono::Utc::now();
    reject_report_in_db(
        &state.db.pool,
        &request.report_id,
        &request.reason,
        &request.validator_notes,
        &request.validated_by,
        &rejection_time,
    )
    .await?;

    tracing::info!(
        report_id = %request.report_id,
        validated_by = %request.validated_by,
        "Successfully rejected report"
    );

    Ok((
        StatusCode::OK,
        Json(RejectReportResponse {
            message: "Report successfully rejected".to_string(),
            report_id: request.report_id,
            status: "rejected".to_string(),
            reason: request.reason,
            validator_notes: request.validator_notes,
            validated_by: request.validated_by,
            rejected_at: rejection_time,
        }),
    ))
}

async fn get_report_by_id(pool: &PgPool, report_id: &Uuid) -> Result<Option<Report>> {
    let report = sqlx::query_as::<_, Report>(
        r#"
        SELECT 
            id, title, project_id, body, reported_by, validated_by,
            status::text as status, 
            severity::text as severity, 
            allocated_reward, 
            reason::text as reason, 
            validator_notes,
            researcher_response, 
            created_at, 
            updated_at
        FROM research_report 
        WHERE id = $1
        "#,
    )
    .bind(report_id)
    .fetch_optional(pool)
    .await?;

    Ok(report)
}

async fn reject_report_in_db(
    pool: &PgPool,
    report_id: &Uuid,
    reason: &str,
    validator_notes: &Option<String>,
    validated_by: &str,
    rejection_time: &chrono::DateTime<chrono::Utc>,
) -> Result<()> {
    let result = sqlx::query(
        r#"
        UPDATE research_report 
        SET 
            status = 'rejected'::report_status_type,
            reason = $2::rejection_reason,
            validator_notes = $3,
            validated_by = $4,
            updated_at = $5
        WHERE id = $1
        "#,
    )
    .bind(report_id)
    .bind(reason)
    .bind(validator_notes)
    .bind(validated_by)
    .bind(rejection_time)
    .execute(pool)
    .await?;

    // Check if any rows were affected (report exists and was updated)
    if result.rows_affected() == 0 {
        return Err(Error::NotFound);
    }

    Ok(())
}
