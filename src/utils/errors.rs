use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Solana error: {0}")]
    Solana(String),
    
    #[error("Ethereum error: {0}")]
    Ethereum(String),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InvalidAddress(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)),
            AppError::Serialization(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Serialization error: {}", e)),
            AppError::Http(e) => (StatusCode::BAD_GATEWAY, format!("HTTP error: {}", e)),
            AppError::Solana(e) => (StatusCode::BAD_GATEWAY, format!("Solana error: {}", e)),
            AppError::Ethereum(e) => (StatusCode::BAD_GATEWAY, format!("Ethereum error: {}", e)),
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {}", e)),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

