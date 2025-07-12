use crate::models::{Alert, AlertStatus, CreateAlertRequest};
use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use std::path::Path;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self> {
        // 从 URL 中提取数据库文件路径
        if let Some(db_path_str) = url.strip_prefix("sqlite:") {
            let db_path = Path::new(db_path_str);
            // 确保父目录存在
            if let Some(parent_dir) = db_path.parent() {
                if !parent_dir.exists() {
                    tracing::info!("创建数据库目录: {}", parent_dir.display());
                    std::fs::create_dir_all(parent_dir)?;
                }
            }
            // 如果数据库文件不存在，先创建一个空文件
            // 这解决了OneDrive环境中SQLite无法自动创建文件的问题
            if !db_path.exists() {
                tracing::info!("创建数据库文件: {}", db_path.display());
                std::fs::File::create(db_path)?;
            }
        }

        let pool = SqlitePool::connect(url).await?;

        tracing::info!("运行数据库迁移...");
        sqlx::migrate!("./migrations").run(&pool).await?;
        tracing::info!("数据库迁移完成");

        Ok(Self { pool })
    }

    pub async fn create_alert(&self, request: &CreateAlertRequest) -> Result<Alert> {
        let symbol = &request.symbol;
        let condition_str = request.condition.to_string().to_lowercase();
        let condition = condition_str.as_str();
        let price = request.price;
        let notification_email = request.notification_email.as_deref();
        let user_id = &request.user_id;
        
        let alert = sqlx::query_as!(
            Alert,
            r#"
            INSERT INTO alerts (symbol, condition, price, status, notification_email, user_id)
            VALUES (?, ?, ?, 'active', ?, ?)
            RETURNING id as "id!", symbol, condition as "condition: _", price, 
                     status as "status: _", created_at, updated_at, triggered_at, 
                     notification_email, 
                     COALESCE(user_id, 'default') as "user_id!"
            "#,
            symbol,
            condition,
            price,
            notification_email,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(alert)
    }

    pub async fn list_alerts(&self) -> Result<Vec<Alert>> {
        let alerts = sqlx::query_as!(
            Alert,
            r#"
            SELECT id as "id!", symbol, condition as "condition: _", price, 
                   status as "status: _", created_at, updated_at, triggered_at, notification_email,
                   COALESCE(user_id, 'default') as "user_id!"
            FROM alerts
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }

    pub async fn list_alerts_by_user(&self, user_id: &str) -> Result<Vec<Alert>> {
        let alerts = sqlx::query_as!(
            Alert,
            r#"
            SELECT id as "id!", symbol, condition as "condition: _", price, 
                   status as "status: _", created_at, updated_at, triggered_at, notification_email,
                   COALESCE(user_id, 'default') as "user_id!"
            FROM alerts
            WHERE COALESCE(user_id, 'default') = ?
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }

    pub async fn get_alert(&self, id: i64) -> Result<Option<Alert>> {
        let alert = sqlx::query_as!(
            Alert,
            r#"
            SELECT id as "id!", symbol, condition as "condition: _", price, 
                   status as "status: _", created_at, updated_at, triggered_at, notification_email,
                   COALESCE(user_id, 'default') as "user_id!"
            FROM alerts
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(alert)
    }

    pub async fn get_alert_by_user(&self, id: i64, user_id: &str) -> Result<Option<Alert>> {
        let alert = sqlx::query_as!(
            Alert,
            r#"
            SELECT id as "id!", symbol, condition as "condition: _", price, 
                   status as "status: _", created_at, updated_at, triggered_at, notification_email,
                   COALESCE(user_id, 'default') as "user_id!"
            FROM alerts
            WHERE id = ? AND COALESCE(user_id, 'default') = ?
            "#,
            id,
            user_id
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

    pub async fn delete_alert_by_user(&self, id: i64, user_id: &str) -> Result<bool> {
        let result = sqlx::query!(
            r#"
            DELETE FROM alerts
            WHERE id = ? AND user_id = ?
            "#,
            id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    pub async fn update_alert(
        &self,
        id: i64,
        request: &CreateAlertRequest,
    ) -> Result<Option<Alert>> {
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

    // 演示模式相关功能
    pub async fn count_alerts_by_user(&self, user_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM alerts WHERE user_id = ?",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    pub async fn cleanup_old_demo_data(&self, retention_hours: u64) -> Result<i64> {
        let retention_hours_str = retention_hours.to_string();
        let result = sqlx::query!(
            r#"
            DELETE FROM alerts 
            WHERE user_id != 'default' 
            AND created_at < datetime('now', '-' || ? || ' hours')
            "#,
            retention_hours_str
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    pub async fn get_active_alerts_for_user(&self, user_id: &str) -> Result<Vec<Alert>> {
        let alerts = sqlx::query_as!(
            Alert,
            r#"
            SELECT id as "id!", symbol, condition as "condition: _", price, 
                   status as "status: _", created_at, updated_at, triggered_at, notification_email,
                   COALESCE(user_id, 'default') as "user_id!"
            FROM alerts
            WHERE COALESCE(user_id, 'default') = ? AND status = 'active'
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }
}

// Re-export common types
pub use sqlx::Error as DbError;
#[allow(dead_code)]
pub type DbResult<T> = Result<T, DbError>;
