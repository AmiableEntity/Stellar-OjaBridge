use axum::Json;
use serde_json::{json, Value};

pub async fn info() -> Json<Value> {
    Json(json!({
        "deposit": {
            "enabled": true,
            "authentication_required": true,
            "fee_fixed": 0,
            "fee_percent": 1.0,
            "min_amount": 1000,
            "max_amount": 1000000
        },
        "withdraw": {
            "enabled": true,
            "authentication_required": true,
            "fee_fixed": 0,
            "fee_percent": 1.0,
            "min_amount": 1000,
            "max_amount": 1000000,
            "types": {
                "bank_account": {
                    "fields": {
                        "account_number": {
                            "description": "Bank account number",
                            "optional": false
                        },
                        "bank_code": {
                            "description": "Bank code",
                            "optional": false
                        },
                        "account_name": {
                            "description": "Account holder name",
                            "optional": false
                        }
                    }
                }
            }
        },
        "fee": {
            "enabled": true,
            "authentication_required": false
        },
        "features": {
            "account_creation": false,
            "claimable_balances": false
        }
    }))
}
