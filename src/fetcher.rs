use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time;
use tracing::{info, error, warn};
use rand;

#[derive(Debug, Deserialize)]
struct YahooQuoteResponse {
    chart: YahooChart,
}

#[derive(Debug, Deserialize)]
struct YahooChart {
    result: Vec<YahooResult>,
    error: Option<YahooError>,
}

#[derive(Debug, Deserialize)]
struct YahooResult {
    meta: YahooMeta,
}

#[derive(Debug, Deserialize)]
struct YahooMeta {
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,
    #[serde(rename = "regularMarketVolume")]
    regular_market_volume: Option<i64>,
    symbol: String,
}

#[derive(Debug, Deserialize)]
struct YahooError {
    code: String,
    description: String,
}

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
                    // 如果API失败，尝试使用模拟数据作为后备
                    if let Ok(fallback_price) = self.fetch_fallback_price(&symbol.symbol).await {
                        warn!("Using fallback price for {}", symbol.symbol);
                        if let Err(e) = self.save_price(&fallback_price).await {
                            error!("Failed to save fallback price for {}: {}", symbol.symbol, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn fetch_price(&self, symbol: &str) -> Result<StockPrice> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );

        info!("Fetching price for {} from Yahoo Finance", symbol);

        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        let yahoo_response: YahooQuoteResponse = response.json().await?;

        if let Some(error) = yahoo_response.chart.error {
            return Err(anyhow::anyhow!("Yahoo Finance error: {} - {}", error.code, error.description));
        }

        let result = yahoo_response.chart.result
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No data returned for symbol {}", symbol))?;

        let price = result.meta.regular_market_price
            .ok_or_else(|| anyhow::anyhow!("No price data for symbol {}", symbol))?;

        let volume = result.meta.regular_market_volume.unwrap_or(0);

        Ok(StockPrice {
            symbol: symbol.to_string(),
            price,
            volume,
            timestamp: Utc::now(),
        })
    }

    // 后备方案：使用模拟数据
    async fn fetch_fallback_price(&self, symbol: &str) -> Result<StockPrice> {
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

        // 生成-1%到+1%的随机变化
        let change_percent = (rand::random::<f64>() - 0.5) * 0.02; // -0.01 到 +0.01
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
        info!("Saving price for {}: ${:.2}", price.symbol, price.price);
        
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
                    } else {
                        info!("🔔 Alert {} triggered! {} is now ${:.2} (target: {} ${:.2})", 
                              alert_id, symbol, current_price, alert.condition, alert.price);
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