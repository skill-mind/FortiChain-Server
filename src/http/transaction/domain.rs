use bigdecimal::{BigDecimal, Zero};
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate)]
pub struct DepositRequest {
    #[garde(custom(validate_starknet_address))]
    pub wallet_address: String,
    #[garde(custom(validate_deposit_amount))]
    pub amount: BigDecimal,
    #[garde(custom(validate_currency))]
    pub currency: String,
    #[garde(inner(length(min = 1, max = 255)))]
    pub notes: Option<String>,
    #[garde(length(equal = 65))]
    pub transaction_hash: String,
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

pub fn validate_deposit_amount(amount: &BigDecimal, _context: &()) -> garde::Result {
    if amount > &BigDecimal::zero() {
        Ok(())
    } else {
        Err(garde::Error::new("Amount must be greater than zero"))
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

// src/transaction/withdraw.rs
// use bigdecimal::{BigDecimal, Zero};
// use garde::Validate;
// use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate)]
pub struct WithdrawalRequest {
    #[garde(custom(validate_starknet_address))]
    pub wallet_address: String,
    #[garde(custom(validate_withdrawal_amount))]
    pub amount: BigDecimal,
    #[garde(custom(validate_currency))]
    pub currency: String,
    #[garde(inner(length(min = 1, max = 255)))]
    pub notes: Option<String>,
    #[garde(length(equal = 65))]
    pub transaction_hash: String,
}

pub fn validate_withdrawal_amount(amount: &BigDecimal, _context: &()) -> garde::Result {
    if amount > &BigDecimal::zero() {
        Ok(())
    } else {
        Err(garde::Error::new(
            "Withdrawal amount must be greater than zero",
        ))
    }
}
