-- Create alerts table
CREATE TABLE IF NOT EXISTS alerts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    condition TEXT NOT NULL CHECK (condition IN ('above', 'below')),
    price REAL NOT NULL,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'triggered', 'cancelled')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    triggered_at DATETIME
);

-- Create index on symbol for faster lookups
CREATE INDEX IF NOT EXISTS idx_alerts_symbol ON alerts(symbol);

-- Create index on status for filtering active alerts
CREATE INDEX IF NOT EXISTS idx_alerts_status ON alerts(status); 