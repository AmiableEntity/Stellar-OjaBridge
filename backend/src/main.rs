use anyhow::{Context, Result};
use axum::{
    routing::get,
    Router,
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod models;
mod routes;
mod services;
mod payouts;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/.well-known/stellar.toml", get(stellar_toml))
        .route("/sep24/info", get(routes::sep24::info))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({
        "status": "healthy",
        "service": "stellar-ojabridge",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

async fn stellar_toml() -> String {
    format!(
        r#"NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
ACCOUNTS=["{}"]
VERSION="0.1.0"

[DOCUMENTATION]
ORG_NAME="Stellar-OjaBridge"
ORG_URL="https://github.com/yourusername/stellar-ojabridge"

[[CURRENCIES]]
code="{}"
issuer="{}"
is_asset_anchored=true
anchor_asset_type="fiat"
anchor_asset="NGN"

[TRANSFER_SERVER_SEP0024]
TRANSFER_SERVER="http://localhost:8080/sep24"
"#,
        std::env::var("STELLAR_DISTRIBUTION_PUBLIC").unwrap_or_default(),
        std::env::var("STELLAR_ASSET_CODE").unwrap_or("NGNT".to_string()),
        std::env::var("STELLAR_ASSET_ISSUER").unwrap_or_default()
    )
}
