use anyhow::Result;
use chrono::Utc;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize)]
pub struct StockPrice {
    pub symbol: String,
    pub price: f64,
    pub volume: i64,
    pub timestamp: chrono::DateTime<Utc>,
}

pub struct PriceFetcher {
    client: reqwest::Client,
    db: SqlitePool,
}

impl PriceFetcher {
    pub fn new(db: SqlitePool) -> Self {
        Self {
            client: reqwest::Client::new(),
            db,
        }
    }

    /// 获取单个股票的实时价格
    pub async fn fetch_price(&self, symbol: &str) -> Result<StockPrice> {
        // Yahoo Finance API URL
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // 解析响应
        let price = response["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or_else(|| anyhow::anyhow!("Failed to get price"))?;

        let volume = response["chart"]["result"][0]["meta"]["regularMarketVolume"]
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("Failed to get volume"))?;

        let stock_price = StockPrice {
            symbol: symbol.to_string(),
            price,
            volume,
            timestamp: Utc::now(),
        };

        // 保存到数据库
        self.save_price(&stock_price).await?;

        Ok(stock_price)
    }

    /// 保存价格到数据库
    async fn save_price(&self, price: &StockPrice) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO price_history (stock_symbol, price, volume, timestamp)
            VALUES (?, ?, ?, ?)
            "#,
            price.symbol,
            price.price,
            price.volume,
            price.timestamp,
        )
        .execute(&self.db)
        .await?;

        // 更新所有相关的活跃提醒的当前价格
        sqlx::query!(
            r#"
            UPDATE alerts
            SET current_price = ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE stock_symbol = ?
              AND status = 'active'
            "#,
            price.price,
            price.symbol,
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// 检查并触发提醒
    pub async fn check_alerts(&self) -> Result<Vec<i64>> {
        let mut triggered_alerts = Vec::new();

        // 获取所有活跃的提醒
        let alerts = sqlx::query!(
            r#"
            SELECT id, stock_symbol, condition, target_price, current_price, email
            FROM alerts
            WHERE status = 'active'
              AND current_price IS NOT NULL
            "#
        )
        .fetch_all(&self.db)
        .await?;

        for alert in alerts {
            let triggered = match alert.condition.as_str() {
                "above" => alert.current_price.unwrap_or_default() > alert.target_price,
                "below" => alert.current_price.unwrap_or_default() < alert.target_price,
                _ => false,
            };

            if triggered {
                // 更新提醒状态
                sqlx::query!(
                    r#"
                    UPDATE alerts
                    SET status = 'triggered',
                        triggered_at = CURRENT_TIMESTAMP
                    WHERE id = ?
                    "#,
                    alert.id
                )
                .execute(&self.db)
                .await?;

                // 记录触发历史
                sqlx::query!(
                    r#"
                    INSERT INTO alert_history 
                    (alert_id, stock_symbol, triggered_price, target_price, email)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    alert.id,
                    alert.stock_symbol,
                    alert.current_price,
                    alert.target_price,
                    alert.email
                )
                .execute(&self.db)
                .await?;

                triggered_alerts.push(alert.id);
            }
        }

        Ok(triggered_alerts.into_iter().flatten().collect())
    }
} 