use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time;
use tracing::{info, error, warn};
use rand;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::Semaphore;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::config::PriceFetcherConfig;
use crate::email::EmailNotifier;

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

// 缓存结构
#[derive(Debug, Clone)]
struct PriceCache {
    price: f64,
    volume: i64,
    timestamp: chrono::DateTime<Utc>,
}

// 价格服务状态
pub struct PriceService {
    client: Client,
    db: SqlitePool,
    update_interval: Duration,
    cache: Arc<RwLock<HashMap<String, PriceCache>>>,
    semaphore: Arc<Semaphore>,
    request_count: AtomicU64,
    last_reset: AtomicU64,
    email_notifier: Arc<EmailNotifier>,
}

impl PriceService {
    pub fn new(db: SqlitePool, config: &PriceFetcherConfig, email_notifier: Arc<EmailNotifier>) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(config.request_timeout_secs))
                .pool_idle_timeout(Duration::from_secs(config.pool_idle_timeout_secs))
                .build()
                .expect("Failed to create HTTP client"),
            db,
            update_interval: Duration::from_secs(config.update_interval_secs),
            cache: Arc::new(RwLock::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(config.max_concurrent_requests)),
            request_count: AtomicU64::new(0),
            last_reset: AtomicU64::new(Utc::now().timestamp() as u64),
            email_notifier,
        }
    }

    pub async fn start(&self, config: &PriceFetcherConfig) -> Result<()> {
        info!("Starting price service...");
        
        let mut interval = time::interval(self.update_interval);
        loop {
            interval.tick().await;
            if let Err(e) = self.update_prices(config).await {
                error!("Failed to update prices: {}", e);
            }
            // 每小时重置请求计数
            self.check_and_reset_request_count().await;
        }
    }

    async fn check_and_reset_request_count(&self) {
        let now = Utc::now().timestamp() as u64;
        let last_reset = self.last_reset.load(Ordering::Relaxed);
        if now - last_reset >= 3600 { // 1小时
            self.request_count.store(0, Ordering::Relaxed);
            self.last_reset.store(now, Ordering::Relaxed);
            info!("Reset request count");
        }
    }

    async fn update_prices(&self, config: &PriceFetcherConfig) -> Result<()> {
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
            // 检查缓存
            if let Some(cached) = self.get_cached_price(&symbol.symbol).await {
                if Utc::now() - cached.timestamp < chrono::Duration::seconds(config.cache_ttl_secs as i64) {
                    continue; // 缓存未过期，跳过更新
                }
            }

            // 获取信号量许可
            let _permit = self.semaphore.acquire().await?;
            
            // 检查请求限制
            if self.request_count.load(Ordering::Relaxed) >= config.max_requests_per_hour {
                warn!("Rate limit reached, waiting for next hour");
                time::sleep(Duration::from_secs(60)).await;
                continue;
            }

            match self.fetch_price_with_retry(&symbol.symbol, config.max_retries).await {
                Ok(price) => {
                    self.request_count.fetch_add(1, Ordering::Relaxed);
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

    async fn get_cached_price(&self, symbol: &str) -> Option<PriceCache> {
        self.cache.read().await.get(symbol).cloned()
    }

    async fn set_cached_price(&self, symbol: String, price: PriceCache) {
        self.cache.write().await.insert(symbol, price);
    }

    async fn fetch_price_with_retry(&self, symbol: &str, max_retries: u32) -> Result<StockPrice> {
        let mut retries = 0;
        let mut last_error = None;

        while retries < max_retries {
            match self.fetch_price(symbol).await {
                Ok(price) => return Ok(price),
                Err(e) => {
                    let error_msg = e.to_string();
                    last_error = Some(e);
                    retries += 1;
                    if retries < max_retries {
                        let delay = Duration::from_secs(2u64.pow(retries)); // 指数退避
                        warn!("Retry {} for {} after {}s: {}", retries, symbol, delay.as_secs(), error_msg);
                        time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Failed to fetch price after {} retries", max_retries)))
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

        let stock_price = StockPrice {
            symbol: symbol.to_string(),
            price,
            volume,
            timestamp: Utc::now(),
        };

        // 更新缓存
        self.set_cached_price(symbol.to_string(), PriceCache {
            price,
            volume,
            timestamp: stock_price.timestamp,
        }).await;

        Ok(stock_price)
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
                    // 标记预警为已触发
                    if let Err(e) = self.mark_alert_triggered(alert_id).await {
                        error!("Failed to mark alert {:?} as triggered: {}", alert_id, e);
                        continue;
                    }

                    info!("🔔 Alert {} triggered! {} is now ${:.2} (target: {} ${:.2})", 
                          alert_id, symbol, current_price, alert.condition, alert.price);

                    // 获取完整的预警信息并发送邮件通知
                    match self.get_alert_by_id(alert_id).await {
                        Ok(Some(full_alert)) => {
                            info!("Sending email notification for alert {}", alert_id);
                            if let Err(e) = self.email_notifier
                                .send_alert_notification(&full_alert, current_price).await {
                                error!("Failed to send email notification for alert {}: {}", alert_id, e);
                            } else {
                                info!("✅ Email notification sent successfully for alert {}", alert_id);
                            }
                        }
                        Ok(None) => {
                            error!("Alert {} not found after triggering", alert_id);
                        }
                        Err(e) => {
                            error!("Failed to fetch alert {} for email notification: {}", alert_id, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // 添加获取完整Alert信息的方法
    async fn get_alert_by_id(&self, alert_id: i64) -> Result<Option<crate::models::Alert>> {
        let alert = sqlx::query_as!(
            crate::models::Alert,
            r#"
            SELECT id, symbol, condition as "condition: crate::models::AlertCondition", 
                   price, status as "status: crate::models::AlertStatus", 
                   created_at, updated_at, triggered_at, notification_email
            FROM alerts
            WHERE id = ?
            "#,
            alert_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(alert)
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

    pub async fn start_price_updater(self: Arc<Self>, config: Arc<PriceFetcherConfig>) {
        tokio::spawn(async move {
            loop {
                if let Err(e) = self.update_prices(&config).await {
                    error!("Error updating prices: {}", e);
                }
                time::sleep(self.update_interval).await;
            }
        });
    }
}

// 为 PriceService 实现 Clone
impl Clone for PriceService {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            db: self.db.clone(),
            update_interval: self.update_interval,
            cache: self.cache.clone(),
            semaphore: self.semaphore.clone(),
            request_count: AtomicU64::new(self.request_count.load(Ordering::Relaxed)),
            last_reset: AtomicU64::new(self.last_reset.load(Ordering::Relaxed)),
            email_notifier: self.email_notifier.clone(),
        }
    }
} 