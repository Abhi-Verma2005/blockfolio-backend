use axum::{extract::{Path, State}, Json, response::IntoResponse};

use crate::state::AppState;
use crate::types::portfolio::PortfolioResponse;
use crate::utils::errors::AppError;

pub async fn get_balances(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // Validate Ethereum address format
    if !is_valid_ethereum_address(&address) {
        return Err(AppError::InvalidAddress(format!("Invalid Ethereum address: {}", address)));
    }

    // Check cache first
    if let Some(cached_data) = state.cache.get_balance(&address, "ethereum").await? {
        let portfolio: PortfolioResponse = serde_json::from_value(cached_data)?;
        return Ok(Json(portfolio).into_response());
    }

    // Cache miss - fetch from RPC
    let portfolio: PortfolioResponse = state.ethereum_client.fetch_portfolio(&address).await?;
    
    // Store in cache
    let ttl_seconds = std::env::var("CACHE_TTL_SECONDS")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .unwrap_or(30);
    state.cache.set_balance(&address, "ethereum", &serde_json::to_value(&portfolio)?, ttl_seconds).await?;

    Ok(Json(portfolio).into_response())
}

fn is_valid_ethereum_address(address: &str) -> bool {
    address.starts_with("0x") && address.len() == 42 && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

