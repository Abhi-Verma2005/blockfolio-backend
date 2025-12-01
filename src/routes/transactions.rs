use axum::{extract::{Path, Query, State}, Json, response::IntoResponse};
use serde::Deserialize;
use crate::state::AppState;
use crate::types::transaction::Transaction;
use crate::utils::errors::AppError;

#[derive(Deserialize)]
pub struct TransactionQuery {
    pub limit: Option<usize>,
}

pub async fn get_solana_transactions(
    Path(address): Path<String>,
    Query(params): Query<TransactionQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let limit = params.limit.unwrap_or(10);
    let transactions = state.solana_client.fetch_transactions(&address, limit).await?;
    Ok(Json(transactions).into_response())
}

pub async fn get_ethereum_transactions(
    Path(address): Path<String>,
    Query(params): Query<TransactionQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let limit = params.limit.unwrap_or(10);
    let transactions = state.ethereum_client.fetch_transactions(&address, limit).await?;
    Ok(Json(transactions).into_response())
}






