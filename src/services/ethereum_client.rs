use anyhow::Result;
use ethers::providers::{Provider, Http, Middleware};
use ethers::types::Address as EthAddress;
use ethers::contract::Contract;
use ethers::abi::Abi;
use std::sync::Arc;
use crate::types::portfolio::PortfolioResponse;
use crate::types::token::Token;
use crate::services::price_service::PriceService;
use crate::services::metadata_service::MetadataService;
use crate::config::Config;

// Popular ERC-20 tokens
const POPULAR_TOKENS: &[(&str, &str)] = &[
    ("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "USDC"),
    ("0xdAC17F958D2ee523a2206206994597C13D831ec7", "USDT"),
    ("0x6B175474E89094C44Da98b954EedeAC495271d0F", "DAI"),
    ("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", "WBTC"),
    ("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", "WETH"),
    ("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", "UNI"),
    ("0x514910771AF9Ca656af840dff83E8264EcF986CA", "LINK"),
    ("0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9", "AAVE"),
];

#[derive(Clone)]
pub struct EthereumClient {
    rpc_url: String,
    price_service: PriceService,
    metadata_service: MetadataService,
    config: Config,
}

impl EthereumClient {
    pub fn new(rpc_url: String, price_service: PriceService, metadata_service: MetadataService, config: Config) -> Self {
        Self {
            rpc_url,
            price_service,
            metadata_service,
            config,
        }
    }

    pub async fn fetch_portfolio(&self, address: &str) -> Result<PortfolioResponse> {
        let addr: EthAddress = address.parse()?;
        
        // Create provider
        let provider = Provider::<Http>::try_from(self.rpc_url.as_str())?;
        
        // Fetch ETH balance
        let balance = provider.get_balance(addr, None).await?;
        let eth_balance = balance.as_u128() as f64 / 1e18;
        
        // Get ETH price
        let (eth_price, _eth_price_change) = self.price_service.get_ethereum_price_with_change("ETH").await.unwrap_or((0.0, None));
        let eth_value = eth_balance * eth_price;

        // Fetch ERC-20 token balances
        let mut tokens = Vec::new();
        
        for (token_address, _symbol) in POPULAR_TOKENS {
            if let Ok((balance, decimals, symbol)) = self.get_token_info(&provider, &addr, token_address).await {
                if balance > 0.0 {
                    let (price, price_change) = self.price_service.get_ethereum_price_with_change(token_address).await.unwrap_or((0.0, None));
                    let value = balance * price;

                    // Get metadata
                    let (name, logo_uri) = self.metadata_service.get_ethereum_metadata(token_address).await.unwrap_or((None, None));

                    tokens.push(Token {
                        symbol: symbol.clone(),
                        mint_or_address: token_address.to_string(),
                        amount: balance,
                        decimals,
                        price_usd: price,
                        value_usd: value,
                        name,
                        logo_uri,
                        price_change_24h: price_change,
                    });
                }
            }
        }

        let total_tokens_count = tokens.len() + if eth_balance > 0.0 { 1 } else { 0 };
        let last_updated = chrono::Utc::now().to_rfc3339();

        Ok(PortfolioResponse {
            chain: "ethereum".to_string(),
            address: address.to_string(),
            native_balance: eth_balance,
            native_price_usd: eth_price,
            native_value_usd: eth_value,
            tokens,
            total_tokens_count: Some(total_tokens_count),
            last_updated: Some(last_updated),
        })
    }

    async fn get_token_info(&self, provider: &Provider<Http>, owner: &EthAddress, token_address: &str) -> Result<(f64, u8, String)> {
        let token_addr: EthAddress = token_address.parse()?;
        let provider_arc = Arc::new(provider.clone());
        
        // ERC-20 ABI
        let abi_json = r#"[
            {
                "constant": true,
                "inputs": [{"name": "_owner", "type": "address"}],
                "name": "balanceOf",
                "outputs": [{"name": "balance", "type": "uint256"}],
                "type": "function"
            },
            {
                "constant": true,
                "inputs": [],
                "name": "decimals",
                "outputs": [{"name": "", "type": "uint8"}],
                "type": "function"
            },
            {
                "constant": true,
                "inputs": [],
                "name": "symbol",
                "outputs": [{"name": "", "type": "string"}],
                "type": "function"
            }
        ]"#;
        
        let abi: Abi = serde_json::from_str(abi_json)?;
        let contract = Contract::new(token_addr, abi, provider_arc);
        
        // Get balance
        let balance: ethers::types::U256 = contract
            .method::<_, ethers::types::U256>("balanceOf", owner.clone())?
            .call()
            .await?;
        
        // Get decimals
        let decimals: u8 = contract
            .method::<_, u8>("decimals", ())?
            .call()
            .await?;
        
        // Get symbol
        let symbol: String = contract
            .method::<_, String>("symbol", ())?
            .call()
            .await?;
        
        let balance_f64 = balance.as_u128() as f64 / 10_f64.powi(decimals as i32);
        
        Ok((balance_f64, decimals, symbol))
    }

    pub async fn fetch_transactions(&self, address: &str, _limit: usize) -> Result<Vec<crate::types::transaction::Transaction>> {
        let _addr: EthAddress = address.parse()?;
        let _provider = Provider::<Http>::try_from(self.rpc_url.as_str())?;
        
        // Get recent transactions
        // Note: This is a simplified implementation
        // In production, you'd use a service like Etherscan API or The Graph
        let transactions = Vec::new();
        
        // For now, return empty list - would need Etherscan API or similar for transaction history
        // Ethereum RPC doesn't provide easy access to transaction history
        
        Ok(transactions)
    }
}

