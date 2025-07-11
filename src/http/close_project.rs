use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use super::types::ClosedProjectRequest;
use crate::AppState;

#[derive(Debug, serde::Serialize)]
pub struct ApiResponse {
    pub message: String,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/closed_project", post(close_project_handler))
}



#[tracing::instrument(name = "closed_project", skip(state, payload))]
async fn close_project_handler(
    state: State<AppState>,
    Json(payload): Json<ClosedProjectRequest>,
) -> axum::http::StatusCode {
   
   let db = &state.db;
    let project_uuid = match Uuid::parse_str(&payload.project_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            tracing::error!("Invalid UUID format: {}", payload.project_id);
            return StatusCode::BAD_REQUEST;
        }
    };

    // validate owner address
    let is_valid_addr = |addr: &str| {
        addr.starts_with("0x")
            && addr.len() == 66
            && addr.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    };
    if !is_valid_addr(&payload.owner_address) || !is_valid_addr(&payload.owner_address) {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid address format".to_string(),
        );
    }

    // check if the user exist

    let user_check_query = r#"
        SELECT wallet_address FROM escrow_users WHERE wallet_address = $1
    "#;

    let user_exists = match db
        .pool
        .fetch_optional(sqlx::query(user_check_query).bind(&payload.owner_address))
        .await
    {
        Ok(Some(_)) => true,
        _ => false,
    };

     if !user_exists {
        tracing::error!("Owner address not found in escrow_users: {}", payload.owner_address);
        return StatusCode::BAD_REQUEST;
    }
    
    // check the user own the project
    let project_query = r#"
        SELECT owner_address, bounty_amount, closed_at FROM projects WHERE id = $1
    "#;

    let project_row = match db.pool.fetch_optional(sqlx.query(project_query)
    .bind(project_uuid)).await  {
        Ok(Some(row)) => row,
        Ok(None) => {
            tracing::error!("Project not found: {}", project_uuid);
            return StatusCode::NOT_FOUND;
        }
        Err(err) => {
            tracing::error!("DB error fetching project: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let owner: String = project_row.get("owner_address");
    let bounty_amount: Option<BigDecimal> = project_row.get("bounty_amount");
    let closed_at: Option<chrono::DateTime<chrono::Utc>> = project_row.get("closed_at");

    if owner != payload.owner_address {
        tracing::warn!("Unauthorized close attempt: {} != {}", owner, payload.owner_address);
        return StatusCode::UNAUTHORIZED;
    }

    if closed_at.is_some() {
        tracing::info!("Project already closed: {}", project_uuid);
        return StatusCode::CONFLICT;
    }

    // Determine how much was disbursed (if any)
      let disbursed_query = r#"
        SELECT COALESCE(SUM(amount), 0) AS disbursed
        FROM escrow_transactions
        WHERE project_id = $1
        AND type = 'bounty_allocation'
    "#;

    let disbursed_row = match db
        .pool
        .fetch_one(sqlx::query(disbursed_query).bind(project_uuid))
        .await
    {
        Ok(row) => row,
        Err(err) => {
            tracing::error!("Error fetching disbursed bounty: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let disbursed: BigDecimal = disbursed_row.get("disbursed");
    let total_bounty = bounty_amount.unwrap_or_else(|| BigDecimal::from(0));
    let refund_amount = total_bounty.clone() - disbursed;

    //  Refund if there's remaining bounty
    if refund_amount > BigDecimal::from(0) {
        let refund_query = r#"
            UPDATE escrow_users
            SET balance = balance + $1, updated_at = now()
            WHERE wallet_address = $2
        "#;

        if let Err(err) = db
            .pool
            .execute(
                sqlx::query(refund_query)
                    .bind(&refund_amount)
                    .bind(&payload.owner_address),
            )
            .await
        {
            tracing::error!("Error refunding bounty to owner: {:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    // MARK PROJECT AS CLOSED
      let close_query = r#"
        UPDATE projects
        SET closed_at = now(), updated_at = now()
        WHERE id = $1
    "#;

    if let Err(err) = db
        .pool
        .execute(sqlx::query(close_query).bind(project_uuid))
        .await
    {
        tracing::error!("Error updating project status: {:?}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    tracing::info!("Project closed successfully: {}", project_uuid);
    StatusCode::OK

    

}

