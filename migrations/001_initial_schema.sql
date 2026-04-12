-- Create transaction status enum
CREATE TYPE transaction_status AS ENUM ('pending', 'completed', 'failed');

-- Create transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stellar_transaction_id VARCHAR(255),
    amount VARCHAR(50) NOT NULL,
    status transaction_status NOT NULL DEFAULT 'pending',
    bank_details JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on stellar_transaction_id for faster lookups
CREATE INDEX idx_transactions_stellar_tx_id ON transactions(stellar_transaction_id);

-- Create index on status for filtering
CREATE INDEX idx_transactions_status ON transactions(status);

-- Create index on created_at for time-based queries
CREATE INDEX idx_transactions_created_at ON transactions(created_at DESC);

-- Create updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE
    ON transactions FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
