use sqlx::{PgPool, Row};
use serde_json::Value;
use chrono::{Utc, Duration};
use anyhow::Result;

#[derive(Clone)]
pub struct CacheService {
    pool: PgPool,
}

impl CacheService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_balance(&self, address: &str, chain: &str) -> Result<Option<Value>> {
        let result = sqlx::query(
            r#"
            SELECT data FROM cached_balances
            WHERE address = $1 AND chain = $2 AND expires_at > NOW()
            "#
        )
        .bind(address)
        .bind(chain)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result
            .map(|row| row.try_get::<Value, _>("data"))
            .transpose()?)
    }

    pub async fn set_balance(&self, address: &str, chain: &str, data: &Value, ttl_seconds: u64) -> Result<()> {
        let expires_at = (Utc::now() + Duration::seconds(ttl_seconds as i64)).naive_utc();

        sqlx::query(
            r#"
            INSERT INTO cached_balances (address, chain, data, expires_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (address, chain) 
            DO UPDATE SET data = $3, expires_at = $4, created_at = NOW()
            "#
        )
        .bind(address)
        .bind(chain)
        .bind(data)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_price(&self, token_id: &str, chain: &str) -> Result<Option<f64>> {
        let result = sqlx::query(
            r#"
            SELECT price_usd FROM cached_prices
            WHERE token_id = $1 AND chain = $2 AND expires_at > NOW()
            "#
        )
        .bind(token_id)
        .bind(chain)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result
            .map(|row| row.try_get::<f64, _>("price_usd"))
            .transpose()?)
    }

    pub async fn get_price_with_change(&self, token_id: &str, chain: &str) -> Result<Option<(f64, Option<f64>)>> {
        let result = sqlx::query(
            r#"
            SELECT price_usd, price_change_24h FROM cached_prices
            WHERE token_id = $1 AND chain = $2 AND expires_at > NOW()
            "#
        )
        .bind(token_id)
        .bind(chain)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let price: f64 = row.try_get("price_usd")?;
                let change: Option<f64> = row.try_get("price_change_24h").ok();
                Ok(Some((price, change)))
            }
            None => Ok(None),
        }
    }

    pub async fn set_price(&self, token_id: &str, chain: &str, price: f64, ttl_seconds: u64) -> Result<()> {
        let expires_at = (Utc::now() + Duration::seconds(ttl_seconds as i64)).naive_utc();

        sqlx::query(
            r#"
            INSERT INTO cached_prices (token_id, chain, price_usd, expires_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (token_id, chain) 
            DO UPDATE SET price_usd = $3, expires_at = $4, created_at = NOW()
            "#
        )
        .bind(token_id)
        .bind(chain)
        .bind(price)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn set_price_with_change(&self, token_id: &str, chain: &str, price: f64, price_change_24h: Option<f64>, ttl_seconds: u64) -> Result<()> {
        let expires_at = (Utc::now() + Duration::seconds(ttl_seconds as i64)).naive_utc();

        sqlx::query(
            r#"
            INSERT INTO cached_prices (token_id, chain, price_usd, price_change_24h, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (token_id, chain) 
            DO UPDATE SET price_usd = $3, price_change_24h = $4, expires_at = $5, created_at = NOW()
            "#
        )
        .bind(token_id)
        .bind(chain)
        .bind(price)
        .bind(price_change_24h)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_metadata(&self, token_id: &str, chain: &str) -> Result<Option<Value>> {
        let result = sqlx::query(
            r#"
            SELECT metadata FROM cached_metadata
            WHERE token_id = $1 AND chain = $2 AND expires_at > NOW()
            "#
        )
        .bind(token_id)
        .bind(chain)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result
            .map(|row| row.try_get::<Value, _>("metadata"))
            .transpose()?)
    }

    pub async fn set_metadata(&self, token_id: &str, chain: &str, metadata: &Value, ttl_seconds: u64) -> Result<()> {
        let expires_at = (Utc::now() + Duration::seconds(ttl_seconds as i64)).naive_utc();

        sqlx::query(
            r#"
            INSERT INTO cached_metadata (token_id, chain, metadata, expires_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (token_id, chain) 
            DO UPDATE SET metadata = $3, expires_at = $4, created_at = NOW()
            "#
        )
        .bind(token_id)
        .bind(chain)
        .bind(metadata)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

