use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{Router, post},
};
use garde::Validate;

use crate::{AppState, Result, http::newsletter::domain::SubscribeNewsletterRequest};

pub fn router() -> Router<AppState> {
    Router::new().route("/subscribe", post(subscribe_handler))
}

#[tracing::instrument(
    name = "Subscribe to newsletter",
    skip(state, req),
    fields(
        subscriber_email = %req.email,
        subscriber_name = %req.name
    )
)]
pub async fn subscribe_handler(
    state: State<AppState>,
    Json(req): Json<SubscribeNewsletterRequest>,
) -> Result<impl IntoResponse> {
    req.validate()?;

    sqlx::query!(
        r#"
        INSERT INTO newsletter_subscribers (email, name)
        VALUES ($1, $2)
        RETURNING id
        "#,
        req.email,
        req.name
    )
    .fetch_one(&state.db.pool)
    .await?;

    Ok(StatusCode::CREATED)
}
