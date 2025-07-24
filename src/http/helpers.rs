use crate::ServiceError;
use bigdecimal::BigDecimal;
use rand::Rng;

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

pub fn check_withdrawal_amount(amount: &BigDecimal) -> Result<(), ServiceError> {
    if amount <= &BigDecimal::from(0) {
        return Err(ServiceError::InvalidAmount);
    }
    Ok(())
}

pub fn check_withdrawal_amount_as_against_balance(
    balance: &BigDecimal,
    withdrawal_amount: &BigDecimal,
) -> Result<(), ServiceError> {
    if balance < withdrawal_amount {
        return Err(ServiceError::InvalidAmount);
    }
    Ok(())
}
