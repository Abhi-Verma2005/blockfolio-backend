-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(100) UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create user_wallets table to link users to their wallet addresses
CREATE TABLE IF NOT EXISTS user_wallets (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    address VARCHAR NOT NULL,
    chain VARCHAR NOT NULL CHECK (chain IN ('solana', 'ethereum')),
    label VARCHAR(100),
    is_primary BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, address, chain)
);

-- Create cached_balances table
CREATE TABLE IF NOT EXISTS cached_balances (
    id SERIAL PRIMARY KEY,
    address VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    UNIQUE(address, chain)
);

-- Create cached_prices table
CREATE TABLE IF NOT EXISTS cached_prices (
    id SERIAL PRIMARY KEY,
    token_id VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    price_usd DECIMAL(20, 8),
    created_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    UNIQUE(token_id, chain)
);

-- Create cached_metadata table
CREATE TABLE IF NOT EXISTS cached_metadata (
    id SERIAL PRIMARY KEY,
    token_id VARCHAR NOT NULL,
    chain VARCHAR NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    expires_at TIMESTAMP NOT NULL,
    UNIQUE(token_id, chain)
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_user_wallets_user_id ON user_wallets(user_id);
CREATE INDEX IF NOT EXISTS idx_user_wallets_address ON user_wallets(address, chain);
CREATE INDEX IF NOT EXISTS idx_cached_balances_lookup ON cached_balances(address, chain, expires_at);
CREATE INDEX IF NOT EXISTS idx_cached_prices_lookup ON cached_prices(token_id, chain, expires_at);
CREATE INDEX IF NOT EXISTS idx_cached_metadata_lookup ON cached_metadata(token_id, chain, expires_at);

