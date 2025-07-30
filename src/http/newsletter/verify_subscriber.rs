use crate::{
    AppState, Error, Result,
    http::newsletter::domain::{
        NewsletterSubscriber, SubscriberStatus, VerifySubscriberRequest, VerifySubscriberResponse,
    },
};
use axum::{Json, extract::State};
use garde::Validate;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Verify Newsletter Subscriber", skip(state))]
pub async fn verify_subscriber(
    State(state): State<AppState>,
    Json(request): Json<VerifySubscriberRequest>,
) -> Result<Json<VerifySubscriberResponse>> {
    request.validate()?;

    tracing::info!(
        token_length = request.token.len(),
        "Attempting to verify newsletter subscriber"
    );

    // Find the subscriber by token
    let subscriber = find_subscriber_by_token(&state.db.pool, &request.token).await?;

    if subscriber.status == SubscriberStatus::Active {
        tracing::warn!(
            subscriber_id = %subscriber.id,
            "Attempt to reverify already active subscriber"
        );
        return Err(Error::unprocessable_entity([(
            "verification",
            "Subscriber is already verified",
        )]));
    }

    let verification_date = chrono::Utc::now();

    match verify_subscriber_in_db(&state.db.pool, subscriber.id, verification_date).await {
        Ok(_) => {
            tracing::info!("Subscriber {} successfully verified", subscriber.id);
            Ok(Json(VerifySubscriberResponse {
                message: "Newsletter subscription successfully verified".to_string(),
                subscriber_id: subscriber.id,
                email: subscriber.email,
                verified_at: verification_date,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to verify subscriber {}: {}", subscriber.id, e);
            Err(Error::unprocessable_entity([(
                "verification",
                "Failed to verify subscriber",
            )]))
        }
    }
}

async fn find_subscriber_by_token(pool: &PgPool, token: &str) -> Result<NewsletterSubscriber> {
    let result = sqlx::query!(
        r#"
        SELECT 
            ns.id,
            ns.email,
            ns.name,
            ns.status as "status!: String",
            ns.subscribed_at,
            ns.created_at,
            ns.updated_at
        FROM newsletter_subscribers ns
        INNER JOIN subscription_token st ON ns.id = st.subscriber_id
        WHERE st.subscription_token = $1
        "#,
        token
    )
    .fetch_optional(pool)
    .await?;

    match result {
        Some(row) => {
            let status = match row.status.as_str() {
                "pending" => SubscriberStatus::Pending,
                "active" => SubscriberStatus::Active,
                "unsubscribed" => SubscriberStatus::Unsubscribed,
                "bounced" => SubscriberStatus::Bounced,
                "spam_complaint" => SubscriberStatus::SpamComplaint,
                _ => {
                    return Err(Error::unprocessable_entity([(
                        "status",
                        "Invalid subscriber status",
                    )]));
                }
            };

            Ok(NewsletterSubscriber {
                id: row.id,
                email: row.email,
                name: row.name,
                status,
                subscribed_at: row.subscribed_at,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
        }
        None => {
            tracing::error!("No subscriber found for token");
            Err(Error::NotFound)
        }
    }
}

async fn verify_subscriber_in_db(
    pool: &PgPool,
    subscriber_id: Uuid,
    verification_date: chrono::DateTime<chrono::Utc>,
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE newsletter_subscribers
        SET
            status = 'active',
            subscribed_at = $2,
            updated_at = $2
        WHERE id = $1
        "#,
        subscriber_id,
        verification_date
    )
    .execute(pool)
    .await?;

    Ok(())
}
