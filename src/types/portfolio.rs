use serde::{Deserialize, Serialize};
use crate::types::token::Token;

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub chain: String,
    pub address: String,
    pub native_balance: f64,
    pub native_price_usd: f64,
    pub native_value_usd: f64,
    pub tokens: Vec<Token>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

