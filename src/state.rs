use sqlx::PgPool;

use crate::services::{
    cache::CacheService,
    ethereum_client::EthereumClient,
    price_service::PriceService,
    solana_client::SolanaClient,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub cache: CacheService,
    pub price_service: PriceService,
    pub solana_client: SolanaClient,
    pub ethereum_client: EthereumClient,
}

