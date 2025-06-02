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
            AlertCondition::Above => write!(f, "高于"),
            AlertCondition::Below => write!(f, "低于"),
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
        match (self, *other) {
            (AlertStatus::Active, "active") => true,
            (AlertStatus::Triggered, "triggered") => true,
            (AlertStatus::Cancelled, "cancelled") => true,
            _ => false,
        }
    }
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