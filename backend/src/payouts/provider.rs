use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PayoutError {
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Invalid bank details: {0}")]
    InvalidBankDetails(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Network error: {0}")]
    NetworkError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutDetails {
    pub account_number: String,
    pub bank_code: String,
    pub account_name: String,
    pub amount: String,
    pub currency: String,
    pub reference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayoutResponse {
    pub transaction_id: String,
    pub status: String,
    pub message: Option<String>,
}

#[async_trait]
pub trait PayoutProvider: Send + Sync {
    async fn send_payout(&self, details: PayoutDetails) -> Result<PayoutResponse, PayoutError>;
    async fn check_status(&self, transaction_id: &str) -> Result<String, PayoutError>;
    fn provider_name(&self) -> &str;
}
