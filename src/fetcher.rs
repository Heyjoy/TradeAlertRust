use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time;
use tracing::{info, error};
use rand;

#[derive(Debug, Deserialize)]
struct StockPrice {
    symbol: String,
    price: f64,
    volume: i64,
    timestamp: chrono::DateTime<Utc>,
}

pub struct PriceFetcher {
    client: Client,
    db: SqlitePool,
    update_interval: Duration,
}

impl PriceFetcher {
    pub fn new(db: SqlitePool, update_interval: Duration) -> Self {
        Self {
            client: Client::new(),
            db,
            update_interval,
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting price fetcher service...");
        
        let mut interval = time::interval(self.update_interval);
        loop {
            interval.tick().await;
            if let Err(e) = self.update_prices().await {
                error!("Failed to update prices: {}", e);
            }
        }
    }

    async fn update_prices(&self) -> Result<()> {
        // 获取所有活跃预警的股票代码
        let symbols = sqlx::query!(
            r#"
            SELECT DISTINCT symbol
            FROM alerts
            WHERE status = 'active'
            "#
        )
        .fetch_all(&self.db)
        .await?;

        for symbol in symbols {
            match self.fetch_price(&symbol.symbol).await {
                Ok(price) => {
                    if let Err(e) = self.save_price(&price).await {
                        error!("Failed to save price for {}: {}", symbol.symbol, e);
                    }
                }
                Err(e) => {
                    error!("Failed to fetch price for {}: {}", symbol.symbol, e);
                }
            }
        }

        Ok(())
    }

    async fn fetch_price(&self, symbol: &str) -> Result<StockPrice> {
        // TODO: 实现实际的股票价格API调用
        // 这里使用模拟数据，价格会有随机变化
        
        // 获取上次价格作为基准
        let last_price = sqlx::query!(
            r#"
            SELECT price
            FROM price_history
            WHERE symbol = ?
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
            symbol
        )
        .fetch_optional(&self.db)
        .await?
        .map(|row| row.price)
        .unwrap_or(100.0); // 如果没有历史价格，默认从100开始

        // 生成-2%到+2%的随机变化
        let change_percent = (rand::random::<f64>() - 0.5) * 0.04; // -0.02 到 +0.02
        let new_price = last_price * (1.0 + change_percent);
        let new_price = (new_price * 100.0).round() / 100.0; // 保留2位小数

        Ok(StockPrice {
            symbol: symbol.to_string(),
            price: new_price,
            volume: (rand::random::<i64>() % 10000) + 1000, // 1000-11000之间的随机成交量
            timestamp: Utc::now(),
        })
    }

    async fn save_price(&self, price: &StockPrice) -> Result<()> {
        // 保存价格历史
        sqlx::query!(
            r#"
            INSERT INTO price_history (symbol, price, volume, timestamp)
            VALUES (?, ?, ?, ?)
            "#,
            price.symbol,
            price.price,
            price.volume,
            price.timestamp,
        )
        .execute(&self.db)
        .await?;

        // 检查并更新相关预警
        self.check_alerts(&price.symbol, price.price).await?;

        Ok(())
    }

    async fn check_alerts(&self, symbol: &str, current_price: f64) -> Result<()> {
        let alerts = sqlx::query!(
            r#"
            SELECT id, condition, price
            FROM alerts
            WHERE symbol = ? AND status = 'active'
            "#,
            symbol
        )
        .fetch_all(&self.db)
        .await?;

        for alert in alerts {
            let triggered = match alert.condition.as_str() {
                "above" => current_price >= alert.price,
                "below" => current_price <= alert.price,
                _ => false,
            };

            if triggered {
                if let Some(alert_id) = alert.id {
                    if let Err(e) = self.mark_alert_triggered(alert_id).await {
                        error!("Failed to mark alert {:?} as triggered: {}", alert_id, e);
                    }
                }
            }
        }

        Ok(())
    }

    async fn mark_alert_triggered(&self, alert_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE alerts
            SET status = 'triggered',
                triggered_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ? AND status = 'active'
            "#,
            alert_id
        )
        .execute(&self.db)
        .await?;

        info!("Alert {} has been triggered", alert_id);
        Ok(())
    }
} 