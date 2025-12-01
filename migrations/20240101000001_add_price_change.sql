-- Add price_change_24h column to cached_prices table
ALTER TABLE cached_prices ADD COLUMN IF NOT EXISTS price_change_24h DECIMAL(10, 4);






