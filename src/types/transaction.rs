use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub hash: String,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub transaction_type: String, // "send" or "receive"
    pub amount: f64,
    pub token_symbol: String,
    pub chain: String,
    pub status: String, // "success" or "failed"
    pub from: String,
    pub to: String,
}




