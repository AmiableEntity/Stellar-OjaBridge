use serde::{Deserialize, Serialize};
use sqlx::types::Json as SqlxJson;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankDetails {
    pub account_number: String,
    pub bank_code: String,
    pub account_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub stellar_transaction_id: Option<String>,
    pub amount: String,
    pub status: TransactionStatus,
    pub bank_details: SqlxJson<BankDetails>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
