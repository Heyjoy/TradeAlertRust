-- Add notification_email column to alerts table for user-specific email notifications
ALTER TABLE alerts ADD COLUMN notification_email TEXT;
 
-- Create index on notification_email for better query performance  
CREATE INDEX IF NOT EXISTS idx_alerts_notification_email ON alerts(notification_email); 