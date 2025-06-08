use sqlx::{sqlite::SqlitePool, Row};
use anyhow::Result;
use crate::models::{Alert, CreateAlertRequest, AlertCondition, AlertStatus};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(url).await?;
        Ok(Self { pool })
    }

    pub async fn create_alert(&self, request: &CreateAlertRequest) -> Result<Alert> {
        let symbol = &request.symbol;
        let condition_str = request.condition.to_string().to_lowercase();
        let condition = condition_str.as_str();
        let price = request.price;
        let notification_email = request.notification_email.as_deref();
        let alert = sqlx::query_as!(
            Alert,
            r#"
            INSERT INTO alerts (symbol, condition, price, status, notification_email)
            VALUES (?, ?, ?, 'active', ?)
            RETURNING id, symbol, condition as "condition: _", price, 
                     status as "status: _", created_at, updated_at, triggered_at, notification_email
            "#,
            symbol,
            condition,
            price,
            notification_email,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(alert)
    }

    pub async fn list_alerts(&self) -> Result<Vec<Alert>> {
        let alerts = sqlx::query_as!(
            Alert,
            r#"
            SELECT id, symbol, condition as "condition: _", price, 
                   status as "status: _", created_at, updated_at, triggered_at, notification_email
            FROM alerts
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }

    pub async fn get_alert(&self, id: i64) -> Result<Option<Alert>> {
        let alert = sqlx::query_as!(
            Alert,
            r#"
            SELECT id, symbol, condition as "condition: _", price, 
                   status as "status: _", created_at, updated_at, triggered_at, notification_email
            FROM alerts
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(alert)
    }

    pub async fn delete_alert(&self, id: i64) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM alerts
            WHERE id = ?
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_alert_status(&self, id: i64, status: AlertStatus) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE alerts
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            status as _,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn mark_alert_triggered(&self, id: i64) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            UPDATE alerts
            SET status = 'triggered', 
                triggered_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND status = 'active'
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn update_alert(&self, id: i64, request: &CreateAlertRequest) -> Result<Option<Alert>> {
        let symbol = &request.symbol;
        let condition_str = request.condition.to_string().to_lowercase();
        let condition = condition_str.as_str();
        let price = request.price;
        let notification_email = request.notification_email.as_deref();
        let result = sqlx::query!(
            r#"
            UPDATE alerts
            SET symbol = ?, condition = ?, price = ?, notification_email = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            symbol,
            condition,
            price,
            notification_email,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            self.get_alert(id).await
        } else {
            Ok(None)
        }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// Re-export common types
pub use sqlx::Error as DbError;
pub type DbResult<T> = Result<T, DbError>; 