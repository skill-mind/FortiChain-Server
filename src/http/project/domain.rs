use bigdecimal::{BigDecimal, Zero};
use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ClosedProjectRequest {
    #[garde(skip)]
    pub project_id: Uuid,
    #[garde(custom(validate_starknet_address))]
    pub owner_address: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyProjectRequest {
    #[garde(url)]
    pub repository_url: String,
    #[garde(custom(validate_starknet_address))]
    pub owner_address: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub owner_address: String,
    pub contract_address: String,
    pub description: String,
    pub is_verified: bool,
    pub verification_date: Option<chrono::DateTime<chrono::Utc>>,
    pub repository_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct VerifyProjectResponse {
    pub message: String,
    pub project_id: Uuid,
    pub verified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[garde(custom(validate_starknet_address))]
    pub owner_address: String,
    #[garde(custom(validate_starknet_address))]
    pub contract_address: String,
    #[garde(ascii, length(bytes, min = 3, max = 256))]
    pub name: String,
    #[garde(length(min = 10, max = 500))]
    pub description: String,
    #[garde(pattern(r#"^[^@\s]+@[^@\s]+\.[^@\s]+$|^https?://.+$"#))]
    pub contact_info: String,
    #[garde(pattern(r#"^(https?|ftp)://[^\s/$.?#].[^\s]*$"#))]
    pub supporting_document_path: Option<String>,
    #[garde(pattern(r#"^(https?|ftp)://[^\s/$.?#].[^\s]*$"#))]
    pub project_logo_path: Option<String>,
    #[garde(pattern(r#"^(https?|ftp)://[^\s/$.?#].[^\s]*$"#))]
    pub repository_url: Option<String>,
    #[garde(length(min = 1), inner(ascii, length(min = 1)))]
    pub tags: Vec<String>,
    #[garde(custom(validate_bounty_amount))]
    pub bounty_amount: Option<BigDecimal>,
    #[garde(custom(validate_bounty_currency))]
    pub bounty_currency: Option<String>,
    #[garde(custom(validate_bounty_expiry_date))]
    pub bounty_expiry_date: Option<DateTime<Utc>>,
}

pub fn validate_starknet_address(addr: &str, _context: &()) -> garde::Result {
    if addr.starts_with("0x")
        && addr.len() == 66
        && addr.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(garde::Error::new("Invalid Address Provided"))
    }
}

pub fn validate_bounty_amount(amount: &Option<BigDecimal>, _ctx: &()) -> garde::Result {
    if amount.is_some() && amount.as_ref().unwrap() <= &BigDecimal::zero() {
        return Err(garde::Error::new("Amount cannot be Negative"));
    }
    Ok(())
}

pub fn validate_bounty_currency(curr: &Option<String>, _context: &()) -> garde::Result {
    let supported_currencies = ["STRK", "USDC", "USDT"];
    if let Some(currency) = curr {
        if !supported_currencies.contains(&currency.as_str()) {
            return Err(garde::Error::new("Invalid Bounty Currency Provided"));
        }
    }
    Ok(())
}

pub fn validate_bounty_expiry_date(date: &Option<DateTime<Utc>>, _context: &()) -> garde::Result {
    if let Some(date) = date {
        if date < &Utc::now() {
            return Err(garde::Error::new("Please Provide Valid Expiry"));
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListProjectsQuery {
    #[garde(custom(validate_starknet_address_optional))]
    pub owner_address: Option<String>,
    #[garde(skip)]
    pub active_only: Option<bool>, // Filter by closed_at being null
    #[garde(skip)]
    pub has_bounty: Option<bool>, // Filter by bounty_amount being not null
    #[garde(custom(validate_sort_by))]
    pub sort_by: Option<String>, // "created_at" (default), "bounty_amount", "name"
    #[garde(custom(validate_sort_order))]
    pub sort_order: Option<String>, // "desc" (default), "asc"
    #[garde(range(min = 1, max = 20))]
    pub limit: Option<i64>, // Max 20, default 10
    #[garde(skip)]
    pub offset: Option<i64>, // Default 0
}

#[derive(Debug, Serialize)]
pub struct ProjectListItem {
    pub id: Uuid,
    pub name: String,
    pub owner_address: String,
    pub contract_address: String,
    pub description: String,
    pub is_verified: bool,
    pub verification_date: Option<DateTime<Utc>>,
    pub repository_url: Option<String>,
    pub bounty_amount: Option<BigDecimal>,
    pub bounty_currency: Option<String>,
    pub bounty_expiry_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ListProjectsResponse {
    pub projects: Vec<ProjectListItem>,
    pub total_count: i64,
    pub has_next: bool,
}

pub fn validate_starknet_address_optional(addr: &Option<String>, _context: &()) -> garde::Result {
    if let Some(addr) = addr {
        validate_starknet_address(addr, _context)?;
    }
    Ok(())
}

pub fn validate_sort_by(sort_by: &Option<String>, _context: &()) -> garde::Result {
    if let Some(sort_by) = sort_by {
        if !["created_at", "bounty_amount", "name"].contains(&sort_by.as_str()) {
            return Err(garde::Error::new("Invalid sort_by field"));
        }
    }
    Ok(())
}

pub fn validate_sort_order(sort_order: &Option<String>, _context: &()) -> garde::Result {
    if let Some(sort_order) = sort_order {
        if !["asc", "desc"].contains(&sort_order.as_str()) {
            return Err(garde::Error::new("Invalid sort_order"));
        }
    }
    Ok(())
}
