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

    let port = std::env::var("BACKEND_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind to {}", addr))?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({
        "status": "healthy",
        "service": "stellar-ojabridge",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

async fn stellar_toml() -> String {
    let network_passphrase = std::env::var("STELLAR_NETWORK_PASSPHRASE")
        .unwrap_or_else(|_| "Test SDF Network ; September 2015".to_string());
    let distribution_public = std::env::var("STELLAR_DISTRIBUTION_PUBLIC")
        .unwrap_or_else(|_| "YOUR_DISTRIBUTION_PUBLIC_KEY".to_string());
    let asset_code = std::env::var("STELLAR_ASSET_CODE").unwrap_or_else(|_| "NGNT".to_string());
    let asset_issuer = std::env::var("STELLAR_ASSET_ISSUER").unwrap_or_default();
    let port = std::env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".to_string());

    format!(
        r#"NETWORK_PASSPHRASE="{network_passphrase}"
ACCOUNTS=["{distribution_public}"]
VERSION="0.1.0"

[DOCUMENTATION]
ORG_NAME="Stellar-OjaBridge"
ORG_URL="https://github.com/yourusername/stellar-ojabridge"

[[CURRENCIES]]
code="{asset_code}"
issuer="{asset_issuer}"
is_asset_anchored=true
anchor_asset_type="fiat"
anchor_asset="NGN"

[TRANSFER_SERVER_SEP0024]
TRANSFER_SERVER="http://localhost:{port}/sep24"
"#,
        network_passphrase = network_passphrase,
        distribution_public = distribution_public,
        asset_code = asset_code,
        asset_issuer = asset_issuer,
        port = port
    )
}
