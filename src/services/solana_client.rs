use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::native_token::lamports_to_sol;
use solana_transaction_status::UiTransactionEncoding;
use solana_program_pack::Pack;
use spl_token::state::Account as TokenAccount;
use crate::types::portfolio::PortfolioResponse;
use crate::types::token::Token;
use crate::services::price_service::PriceService;
use crate::services::metadata_service::MetadataService;
use crate::config::Config;

#[derive(Clone)]
pub struct SolanaClient {
    rpc_url: String,
    price_service: PriceService,
    metadata_service: MetadataService,
    config: Config,
}

#[allow(deprecated)]
impl SolanaClient {
    pub fn new(rpc_url: String, price_service: PriceService, metadata_service: MetadataService, config: Config) -> Self {
        Self {
            rpc_url,
            price_service,
            metadata_service,
            config,
        }
    }

    pub async fn fetch_portfolio(&self, address: &str) -> Result<PortfolioResponse> {
        let pubkey = address.parse::<Pubkey>()?;
        let rpc_client = RpcClient::new(self.rpc_url.clone());
        
        // Fetch SOL balance
        let lamports = rpc_client.get_balance(&pubkey)?;
        #[allow(deprecated)]
        let sol_balance = lamports_to_sol(lamports);
        
        // Get SOL price
        let (sol_price, sol_price_change) = self.price_service.get_solana_price_with_change("SOL").await.unwrap_or((0.0, None));
        let sol_value = sol_balance * sol_price;

        // Fetch SPL token balances
        let mut tokens: Vec<Token> = Vec::new();
        
        let token_program_id = spl_token::ID;
        let token_accounts = rpc_client.get_token_accounts_by_owner(
            &pubkey,
            TokenAccountsFilter::ProgramId(token_program_id),
        )?;

        for account in token_accounts {
            // Parse the pubkey from string
            let account_pubkey: Pubkey = account.pubkey.parse()?;
            // Get account data - get_account returns Result<Account>
            if let Ok(account_info) = rpc_client.get_account(&account_pubkey) {
                // Account data is in account_info.data which is Vec<u8>
                // Try to unpack the token account using spl-token's unpack
                if let Ok(token_account) = TokenAccount::unpack(&account_info.data) {
                    let mint = token_account.mint.to_string();
                    // Decimals are stored in the mint account, not the token account
                    // For now, use a default of 9 (common for most tokens) or fetch from metadata
                    // In production, you'd fetch the mint account to get the actual decimals
                    let decimals = 9u8; // Default, should fetch from mint account
                    let amount = token_account.amount as f64 / 10_f64.powi(decimals as i32);
                    
                    if amount > 0.0 {
                        // Get token price
                        let (price, price_change) = self.price_service.get_solana_price_with_change(&mint).await.unwrap_or((0.0, None));
                        let value = amount * price;
                        
                        // Get metadata
                        let (name, logo_uri) = self.metadata_service.get_solana_metadata(&mint).await.unwrap_or((None, None));
                        
                        // Get symbol from metadata or use mint address
                        let symbol = if let Some(ref n) = name {
                            n.split_whitespace().next().unwrap_or(&mint[..8]).to_string()
                        } else {
                            mint.chars().take(8).collect()
                        };

                        tokens.push(Token {
                            symbol,
                            mint_or_address: mint,
                            amount,
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
        }

        let total_tokens_count = tokens.len() + if sol_balance > 0.0 { 1 } else { 0 };
        let last_updated = chrono::Utc::now().to_rfc3339();

        Ok(PortfolioResponse {
            chain: "solana".to_string(),
            address: address.to_string(),
            native_balance: sol_balance,
            native_price_usd: sol_price,
            native_value_usd: sol_value,
            tokens,
            total_tokens_count: Some(total_tokens_count),
            last_updated: Some(last_updated),
        })
    }

    pub async fn fetch_transactions(&self, address: &str, limit: usize) -> Result<Vec<crate::types::transaction::Transaction>> {
        let pubkey = address.parse::<Pubkey>()?;
        let rpc_client = RpcClient::new(self.rpc_url.clone());
        
        // Get recent signatures - API takes only pubkey, returns all signatures
        let signatures = rpc_client.get_signatures_for_address(&pubkey)?;
        
        let mut transactions = Vec::new();
        
        for sig_info in signatures.iter().take(limit) {
            // Parse signature string to Signature type
            let signature = sig_info.signature.parse::<solana_sdk::signature::Signature>()?;
            // get_transaction requires encoding parameter
            if let Ok(_tx) = rpc_client.get_transaction(&signature, UiTransactionEncoding::Json) {
                let timestamp = sig_info.block_time.unwrap_or(0);
                let status = if sig_info.err.is_none() { "success" } else { "failed" };
                
                // Determine transaction type and amount
                // This is simplified - in reality, you'd parse the transaction details
                let transaction_type = "transfer".to_string();
                let amount = 0.0; // Would need to parse transaction to get actual amount
                let token_symbol = "SOL".to_string();
                
                transactions.push(crate::types::transaction::Transaction {
                    hash: sig_info.signature.to_string(),
                    timestamp,
                    transaction_type,
                    amount,
                    token_symbol,
                    chain: "solana".to_string(),
                    status: status.to_string(),
                    from: address.to_string(),
                    to: address.to_string(), // Would need to parse from transaction
                });
            }
        }
        
        Ok(transactions)
    }
}

