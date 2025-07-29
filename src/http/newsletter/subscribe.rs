use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{Router, post},
};
use garde::Validate;
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::{AppState, Result, db::Db, http::newsletter::domain::NewsletterSubscriber};

pub fn router() -> Router<AppState> {
    Router::new().route("/subscribe", post(subscribe_handler))
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubscribeRequest {
    #[garde(email)]
    email: String,
    #[garde(length(min = 2, max = 255))]
    name: String,
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
    Json(req): Json<SubscribeRequest>,
) -> Result<impl IntoResponse> {
    req.validate()?;
    let subscriber = NewsletterSubscriber {
        email: req.email,
        name: req.name,
    };

    Db::add_subscriber(&state.db.pool, &subscriber).await?;

    Ok(StatusCode::OK)
}

impl Db {
    pub async fn add_subscriber(pool: &PgPool, subscriber: &NewsletterSubscriber) -> Result<Uuid> {
        let result = sqlx::query!(
            r#"
            INSERT INTO newsletter_subscribers (email, name)
            VALUES ($1, $2)
            RETURNING id
            "#,
            subscriber.email,
            subscriber.name
        )
        .fetch_one(pool)
        .await?;

        Ok(result.id)
    }
}
