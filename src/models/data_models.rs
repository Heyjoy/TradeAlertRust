use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

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
    pub notification_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum AlertCondition {
    Above,
    Below,
}

impl fmt::Display for AlertCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlertCondition::Above => write!(f, "Above"),
            AlertCondition::Below => write!(f, "Below"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum AlertStatus {
    Active,
    Triggered,
    Cancelled,
}

impl fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlertStatus::Active => write!(f, "active"),
            AlertStatus::Triggered => write!(f, "triggered"),
            AlertStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl PartialEq<&str> for AlertStatus {
    fn eq(&self, other: &&str) -> bool {
        matches!(
            (self, *other),
            (AlertStatus::Active, "active")
                | (AlertStatus::Triggered, "triggered")
                | (AlertStatus::Cancelled, "cancelled")
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAlertRequest {
    pub symbol: String,
    pub condition: AlertCondition,
    pub price: f64,
    pub notification_email: Option<String>,
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
    pub notification_email: Option<String>,
}

// 用于模板渲染的 Alert 结构体
#[derive(Debug)]
pub struct AlertForTemplate {
    pub id: i64,
    pub symbol: String,
    pub condition: String,
    pub price: f64,
    pub status: String,
    pub created_at: String,
    #[allow(dead_code)]
    pub updated_at: String,
    pub triggered_at: Option<String>,
    pub notification_email: Option<String>,
}

impl From<Alert> for AlertForTemplate {
    fn from(alert: Alert) -> Self {
        Self {
            id: alert.id,
            symbol: alert.symbol,
            condition: alert.condition.to_string(),
            price: alert.price,
            status: alert.status.to_string(),
            created_at: alert.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: alert.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            triggered_at: alert
                .triggered_at
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            notification_email: alert.notification_email,
        }
    }
}

impl From<Alert> for AlertResponse {
    fn from(alert: Alert) -> Self {
        Self {
            id: alert.id,
            symbol: alert.symbol,
            condition: alert.condition,
            price: alert.price,
            status: alert.status,
            created_at: alert.created_at,
            updated_at: alert.updated_at,
            triggered_at: alert.triggered_at,
            notification_email: alert.notification_email,
        }
    }
}

// 实现一些辅助方法
impl Alert {
    #[allow(dead_code)]
    pub fn is_triggered(&self, current_price: f64) -> bool {
        match self.condition {
            AlertCondition::Above => current_price >= self.price,
            AlertCondition::Below => current_price <= self.price,
        }
    }
}
