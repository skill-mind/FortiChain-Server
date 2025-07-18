use tracing::info;
use uuid::Uuid;

/// Dummy notification — replace with real email/queue logic
pub async fn notify_validators(report_id: Uuid, project_id: Uuid) {
    info!("Notifying validators: report={} project={}", report_id, project_id);
    // e.g., send email or push to a message queue
}
