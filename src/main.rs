mod config;
mod database;
mod routes;
mod services;
mod state;
mod types;
mod utils;

use axum::{
    routing::{get, post, delete},
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

use config::Config;
use database::create_pool;
use services::cache::CacheService;
use services::price_service::PriceService;
use services::metadata_service::MetadataService;
use services::solana_client::SolanaClient;
use services::ethereum_client::EthereumClient;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::from_env()?;

    // Create database pool
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Initialize services
    let cache = CacheService::new(pool.clone());
    let price_service = PriceService::new(cache.clone());
    let metadata_service = MetadataService::new(cache.clone());
    let solana_client = SolanaClient::new(
        config.solana_rpc_url.clone(),
        price_service.clone(),
        metadata_service.clone(),
        config.clone(),
    );
    let ethereum_client = EthereumClient::new(
        config.ethereum_rpc_url.clone(),
        price_service.clone(),
        metadata_service.clone(),
        config.clone(),
    );

    let app_state = AppState {
        pool: pool.clone(),
        cache,
        price_service,
        solana_client,
        ethereum_client,
    };

    // Build application with routes
    let app = Router::new()
        .route("/health", get(routes::health::health_check))
        .route("/solana/balances/:address", get(routes::solana::get_balances))
        .route("/ethereum/balances/:address", get(routes::ethereum::get_balances))
        .route("/solana/transactions/:address", get(routes::transactions::get_solana_transactions))
        .route("/ethereum/transactions/:address", get(routes::transactions::get_ethereum_transactions))
        .route("/users", post(routes::users::create_user))
        .route("/users/:user_id", get(routes::users::get_user))
        .route("/users/:user_id/wallets", get(routes::users::get_user_wallets))
        .route("/users/:user_id/wallets", post(routes::users::add_wallet))
        .route("/users/:user_id/wallets/:wallet_id", delete(routes::users::remove_wallet))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
    tracing::info!("Server listening on port {}", config.port);
    
    axum::serve(listener, app).await?;

    Ok(())
}
