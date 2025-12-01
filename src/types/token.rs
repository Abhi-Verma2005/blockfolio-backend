use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub symbol: String,
    pub mint_or_address: String,
    pub amount: f64,
    pub decimals: u8,
    pub price_usd: f64,
    pub value_usd: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_change_24h: Option<f64>,
}

