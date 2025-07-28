use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteValidatorProfileRequest {
    #[garde(custom(validate_starknet_address))]
    pub wallet_address: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteValidatorProfileResponse {
    pub message: String,
    pub validator_id: Uuid,
    pub deleted_at: chrono::DateTime<chrono::Utc>,
}

pub fn validate_starknet_address(address: &str, _context: &()) -> garde::Result {
    if address.starts_with("0x")
        && address.len() == 66
        && address.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(garde::Error::new("Invalid Starknet address"))
    }
}
