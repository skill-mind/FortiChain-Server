use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, sqlx::Type, serde::Serialize)]
#[sqlx(type_name = "subscriber_status", rename_all = "lowercase")]
pub enum SubscriberStatus {
    Pending,
    Active,
    Unsubscribed,
    Bounced,
    SpamComplaint,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NewsletterSubscriber {
    pub id: Uuid,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 2, max = 255))]
    pub name: String,
    pub status: SubscriberStatus,
    pub subscribed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifySubscriberRequest {
    #[garde(ascii, length(min = 1))]
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct VerifySubscriberResponse {
    pub message: String,
    pub subscriber_id: Uuid,
    pub email: String,
    pub verified_at: chrono::DateTime<chrono::Utc>,
}
