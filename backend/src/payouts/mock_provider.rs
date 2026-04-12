use async_trait::async_trait;
use uuid::Uuid;
use super::{PayoutProvider, PayoutDetails, PayoutResponse, PayoutError};

pub struct MockPayoutProvider;

impl MockPayoutProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PayoutProvider for MockPayoutProvider {
    async fn send_payout(&self, details: PayoutDetails) -> Result<PayoutResponse, PayoutError> {
        tracing::info!("Mock payout to {} for {} {}", 
            details.account_number, 
            details.amount, 
            details.currency
        );
        
        Ok(PayoutResponse {
            transaction_id: Uuid::new_v4().to_string(),
            status: "completed".to_string(),
            message: Some("Mock payout successful".to_string()),
        })
    }

    async fn check_status(&self, transaction_id: &str) -> Result<String, PayoutError> {
        tracing::info!("Checking status for transaction: {}", transaction_id);
        Ok("completed".to_string())
    }

    fn provider_name(&self) -> &str {
        "MockProvider"
    }
}
