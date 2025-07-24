use bigdecimal::{BigDecimal, Zero};
use chrono::{DateTime, Utc};
use garde::Validate;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AllocateBountyRequest {
    #[garde(custom(validate_starknet_address))]
    pub wallet_address: String,
    #[garde(custom(validate_starknet_address))]
    pub project_contract_address: String,
    #[garde(custom(validate_amount))]
    pub amount: BigDecimal,
    #[garde(ascii, custom(validate_currency))]
    pub currency: String,
    #[garde(custom(validate_bounty_expiry_date))]
    pub bounty_expiry_date: Option<DateTime<Utc>>, // ISO8601 string
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

pub fn validate_amount(amount: &BigDecimal, _context: &()) -> garde::Result {
    if amount > &BigDecimal::zero() {
        Ok(())
    } else {
        Err(garde::Error::new("Amount must be positive"))
    }
}

pub fn validate_currency(currency: &str, _context: &()) -> garde::Result {
    let supported_currencies = ["STRK", "USDC", "USDT"];
    if supported_currencies.contains(&currency) {
        Ok(())
    } else {
        Err(garde::Error::new("Unsupported currency"))
    }
}

pub fn validate_bounty_expiry_date(date: &Option<DateTime<Utc>>, _context: &()) -> garde::Result {
    if let Some(expiry_date) = date {
        if expiry_date > &Utc::now() {
            Ok(())
        } else {
            Err(garde::Error::new(
                "Bounty expiry date must be in the future",
            ))
        }
    } else {
        Ok(())
    }
}

pub fn generate_transaction_hash() -> String {
    let charset = b"abcdefABCDEF0123456789";
    let mut rng = rand::rng();
    let hash: String = (0..98)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    format!("0x{hash}")
}
