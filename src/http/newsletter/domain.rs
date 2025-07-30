use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NewsletterSubscriber {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 2, max = 255))]
    pub name: String,
}
