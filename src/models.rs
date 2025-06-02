use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Alert {
    pub id: i64,
    pub symbol: String,
    pub condition: AlertCondition,
    pub price: f64,
    pub status: AlertStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub triggered_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum AlertCondition {
    Above,
    Below,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum AlertStatus {
    Active,
    Triggered,
    Cancelled,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlertRequest {
    pub symbol: String,
    pub condition: AlertCondition,
    pub price: f64,
}

#[derive(Debug, Serialize)]
pub struct AlertResponse {
    pub id: i64,
    pub symbol: String,
    pub condition: AlertCondition,
    pub price: f64,
    pub status: AlertStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub triggered_at: Option<NaiveDateTime>,
}

impl From<Alert> for AlertResponse {
    fn from(alert: Alert) -> Self {
        AlertResponse {
            id: alert.id,
            symbol: alert.symbol,
            condition: alert.condition,
            price: alert.price,
            status: alert.status,
            created_at: alert.created_at,
            updated_at: alert.updated_at,
            triggered_at: alert.triggered_at,
        }
    }
}

// 实现一些辅助方法
impl Alert {
    pub fn is_triggered(&self, current_price: f64) -> bool {
        match self.condition {
            AlertCondition::Above => current_price >= self.price,
            AlertCondition::Below => current_price <= self.price,
        }
    }
} 