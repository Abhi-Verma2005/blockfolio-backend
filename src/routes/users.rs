use axum::{extract::{Path, State}, Json, response::IntoResponse};

use crate::state::AppState;
use crate::types::user::{User, CreateUserRequest, UserWallet, AddWalletRequest};
use crate::utils::errors::AppError;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user: User = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, username)
        VALUES ($1, $2)
        RETURNING id, email, username, created_at, updated_at
        "#
    )
    .bind(&payload.email)
    .bind(&payload.username)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(user).into_response())
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user: Option<User> = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, created_at, updated_at
        FROM users
        WHERE id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await?;

    match user {
        Some(u) => Ok(Json(u).into_response()),
        None => Err(AppError::InvalidAddress(format!("User not found: {}", user_id))),
    }
}

pub async fn get_user_wallets(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let wallets: Vec<UserWallet> = sqlx::query_as::<_, UserWallet>(
        r#"
        SELECT id, user_id, address, chain, label, is_primary, created_at
        FROM user_wallets
        WHERE user_id = $1
        ORDER BY is_primary DESC, created_at ASC
        "#
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(wallets).into_response())
}

pub async fn add_wallet(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(payload): Json<AddWalletRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate chain
    if payload.chain != "solana" && payload.chain != "ethereum" {
        return Err(AppError::InvalidAddress(format!("Invalid chain: {}", payload.chain)));
    }

    // Validate address format based on chain
    if payload.chain == "solana" {
        if !is_valid_solana_address(&payload.address) {
            return Err(AppError::InvalidAddress(format!("Invalid Solana address: {}", payload.address)));
        }
    } else if payload.chain == "ethereum" {
        if !is_valid_ethereum_address(&payload.address) {
            return Err(AppError::InvalidAddress(format!("Invalid Ethereum address: {}", payload.address)));
        }
    }

    // If this is set as primary, unset other primary wallets for this user/chain
    if payload.is_primary.unwrap_or(false) {
        sqlx::query!(
            r#"
            UPDATE user_wallets
            SET is_primary = FALSE
            WHERE user_id = $1 AND chain = $2
            "#,
            user_id,
            payload.chain
        )
        .execute(&state.pool)
        .await?;
    }

    let wallet: UserWallet = sqlx::query_as::<_, UserWallet>(
        r#"
        INSERT INTO user_wallets (user_id, address, chain, label, is_primary)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (user_id, address, chain) DO UPDATE
        SET label = COALESCE(EXCLUDED.label, user_wallets.label),
            is_primary = COALESCE(EXCLUDED.is_primary, user_wallets.is_primary)
        RETURNING id, user_id, address, chain, label, is_primary, created_at
        "#
    )
    .bind(user_id)
    .bind(&payload.address)
    .bind(&payload.chain)
    .bind(&payload.label)
    .bind(payload.is_primary.unwrap_or(false))
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(wallet).into_response())
}

pub async fn remove_wallet(
    State(state): State<AppState>,
    Path((user_id, wallet_id)): Path<(i32, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM user_wallets
        WHERE id = $1 AND user_id = $2
        RETURNING id
        "#
    )
    .bind(wallet_id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await?;

    match result {
        Some(_) => Ok(Json(serde_json::json!({ "success": true })).into_response()),
        None => Err(AppError::InvalidAddress("Wallet not found or access denied".to_string())),
    }
}

fn is_valid_solana_address(address: &str) -> bool {
    use bs58;
    bs58::decode(address).into_vec().is_ok() && address.len() >= 32 && address.len() <= 44
}

fn is_valid_ethereum_address(address: &str) -> bool {
    address.starts_with("0x") && address.len() == 42 && address[2..].chars().all(|c| c.is_ascii_hexdigit())
}

