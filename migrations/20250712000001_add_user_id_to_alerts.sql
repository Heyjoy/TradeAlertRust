-- Add user_id column to alerts table for multi-user support
ALTER TABLE alerts ADD COLUMN user_id TEXT DEFAULT 'default';

-- Create index for efficient user-based queries
CREATE INDEX idx_alerts_user_id ON alerts(user_id);

-- Create index for user_id + status queries
CREATE INDEX idx_alerts_user_status ON alerts(user_id, status);