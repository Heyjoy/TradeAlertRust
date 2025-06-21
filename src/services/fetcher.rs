use crate::config::PriceFetcherConfig;
use crate::services::email::EmailNotifier;
use anyhow::Result;
use chrono::Utc;

use reqwest::Client;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::sync::Semaphore;
use tokio::time;
use tracing::{error, info, warn};

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
    #[allow(dead_code)]
    symbol: String,
    #[serde(rename = "shortName")]
    short_name: Option<String>,
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
    name_en: Option<String>,
}

// 缓存结构
#[derive(Debug, Clone)]
struct PriceCache {
    #[allow(dead_code)]
    price: f64,
    #[allow(dead_code)]
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
    pub fn new(
        db: SqlitePool,
        config: &PriceFetcherConfig,
        email_notifier: Arc<EmailNotifier>,
    ) -> Self {
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    async fn check_and_reset_request_count(&self) {
        let now = Utc::now().timestamp() as u64;
        let last_reset = self.last_reset.load(Ordering::Relaxed);
        if now - last_reset >= 3600 {
            // 1小时
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
                if Utc::now() - cached.timestamp
                    < chrono::Duration::seconds(config.cache_ttl_secs as i64)
                {
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

            match self
                .fetch_price_with_retry(&symbol.symbol, config.max_retries)
                .await
            {
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
                        warn!(
                            "Retry {} for {} after {}s: {}",
                            retries,
                            symbol,
                            delay.as_secs(),
                            error_msg
                        );
                        time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            anyhow::anyhow!("Failed to fetch price after {} retries", max_retries)
        }))
    }

    async fn fetch_price(&self, symbol: &str) -> Result<StockPrice> {
        // 根据股票代码判断市场类型
        if self.is_china_stock(symbol) {
            self.fetch_china_stock_price(symbol).await
        } else {
            self.fetch_us_stock_price(symbol).await
        }
    }

    // 判断是否为A股
    fn is_china_stock(&self, symbol: &str) -> bool {
        symbol.ends_with(".SZ") || symbol.ends_with(".SS")
    }

    // 获取A股价格 - 使用新浪财经API
    async fn fetch_china_stock_price(&self, symbol: &str) -> Result<StockPrice> {
        // 转换股票代码格式
        let sina_symbol = self.convert_to_sina_format(symbol);
        let url = format!("https://hq.sinajs.cn/list={}", sina_symbol);

        info!("Fetching A-share price for {} from Sina Finance", symbol);

        let response = self
            .client
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        if !response.status().is_success() {
            // 如果新浪API失败，尝试腾讯API作为备用
            return self.fetch_china_stock_price_tencent(symbol).await;
        }

        let text = response.text().await?;
        if let Some(stock_price) = self.parse_sina_response(&text, symbol)? {
            // 更新缓存
            self.set_cached_price(
                symbol.to_string(),
                PriceCache {
                    price: stock_price.price,
                    volume: stock_price.volume,
                    timestamp: stock_price.timestamp,
                },
            )
            .await;

            Ok(stock_price)
        } else {
            // 新浪API解析失败，尝试腾讯API
            self.fetch_china_stock_price_tencent(symbol).await
        }
    }

    // 腾讯财经API作为A股数据的备用源
    async fn fetch_china_stock_price_tencent(&self, symbol: &str) -> Result<StockPrice> {
        let tencent_symbol = self.convert_to_tencent_format(symbol);
        let url = format!("https://qt.gtimg.cn/q={}", tencent_symbol);

        info!(
            "Fetching A-share price for {} from Tencent Finance (fallback)",
            symbol
        );

        let response = self
            .client
            .get(&url)
            .header("Referer", "https://stockapp.finance.qq.com")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        let text = response.text().await?;
        if let Some(stock_price) = self.parse_tencent_response(&text, symbol)? {
            // 更新缓存
            self.set_cached_price(
                symbol.to_string(),
                PriceCache {
                    price: stock_price.price,
                    volume: stock_price.volume,
                    timestamp: stock_price.timestamp,
                },
            )
            .await;

            Ok(stock_price)
        } else {
            Err(anyhow::anyhow!(
                "Failed to parse Tencent response for {}",
                symbol
            ))
        }
    }

    // 获取美股价格 - 保持原有的Yahoo Finance API
    async fn fetch_us_stock_price(&self, symbol: &str) -> Result<StockPrice> {
        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );

        info!("Fetching US stock price for {} from Yahoo Finance", symbol);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        let yahoo_response: YahooQuoteResponse = response.json().await?;

        if let Some(error) = yahoo_response.chart.error {
            return Err(anyhow::anyhow!(
                "Yahoo Finance error: {} - {}",
                error.code,
                error.description
            ));
        }

        let result = yahoo_response
            .chart
            .result
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No data returned for symbol {}", symbol))?;

        let price = result
            .meta
            .regular_market_price
            .ok_or_else(|| anyhow::anyhow!("No price data for symbol {}", symbol))?;

        let volume = result.meta.regular_market_volume.unwrap_or(0);
        let name_en = result.meta.short_name.clone();

        let stock_price = StockPrice {
            symbol: symbol.to_string(),
            price,
            volume,
            timestamp: Utc::now(),
            name_en,
        };

        // 更新缓存
        self.set_cached_price(
            symbol.to_string(),
            PriceCache {
                price,
                volume,
                timestamp: stock_price.timestamp,
            },
        )
        .await;

        Ok(stock_price)
    }

    // A股股票代码格式转换
    fn convert_to_sina_format(&self, symbol: &str) -> String {
        if symbol.ends_with(".SZ") {
            format!("sz{}", &symbol[..6])
        } else if symbol.ends_with(".SS") {
            format!("sh{}", &symbol[..6])
        } else {
            symbol.to_string()
        }
    }

    fn convert_to_tencent_format(&self, symbol: &str) -> String {
        if symbol.ends_with(".SZ") {
            format!("sz{}", &symbol[..6])
        } else if symbol.ends_with(".SS") {
            format!("sh{}", &symbol[..6])
        } else {
            symbol.to_string()
        }
    }

    // 解析新浪财经API响应
    fn parse_sina_response(&self, text: &str, symbol: &str) -> Result<Option<StockPrice>> {
        // 新浪API返回格式: var hq_str_sz000001="平安银行,27.55,27.25,26.91,27.60,26.20,26.91,26.92,22114263,589824680,..."
        if let Some(start) = text.find('"') {
            if let Some(end) = text.rfind('"') {
                let data_str = &text[start + 1..end];
                let parts: Vec<&str> = data_str.split(',').collect();

                if parts.len() >= 32 {
                    let name = parts[0].to_string();
                    let current_price: f64 = parts[3]
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Failed to parse current price: {}", e))?;
                    let _prev_close: f64 = parts[2]
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Failed to parse previous close: {}", e))?;
                    let volume: i64 = parts[8]
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Failed to parse volume: {}", e))?;

                    return Ok(Some(StockPrice {
                        symbol: symbol.to_string(),
                        price: current_price,
                        volume,
                        timestamp: Utc::now(),
                        name_en: Some(name),
                    }));
                }
            }
        }
        Ok(None)
    }

    // 解析腾讯财经API响应
    fn parse_tencent_response(&self, text: &str, symbol: &str) -> Result<Option<StockPrice>> {
        // 腾讯API返回格式: v_sz000001="51~平安银行~000001~11.84~11.70~11.84~..."
        if let Some(start) = text.find('"') {
            if let Some(end) = text.rfind('"') {
                let data_str = &text[start + 1..end];
                let parts: Vec<&str> = data_str.split('~').collect();

                if parts.len() >= 50 {
                    let name = parts[1].to_string();
                    let current_price: f64 = parts[3]
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Failed to parse current price: {}", e))?;
                    let volume: i64 = parts[6]
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!("Failed to parse volume: {}", e))?
                        as i64;

                    return Ok(Some(StockPrice {
                        symbol: symbol.to_string(),
                        price: current_price,
                        volume,
                        timestamp: Utc::now(),
                        name_en: Some(name),
                    }));
                }
            }
        }
        Ok(None)
    }

    // 后备方案：使用模拟数据
    async fn fetch_fallback_price(&self, symbol: &str) -> Result<StockPrice> {
        // 获取上次价格作为基准
        let last_price = sqlx::query!(
            r#"
            SELECT close_price as price
            FROM price_history
            WHERE symbol = ?
            ORDER BY date DESC
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
            name_en: Some(format!("{} Corporation", symbol)), // 为模拟数据提供一个通用公司名
        })
    }

    async fn save_price(&self, price: &StockPrice) -> Result<()> {
        info!(
            "Saving price for {} ({}): ${:.2}",
            price.symbol,
            price.name_en.as_deref().unwrap_or("Unknown"),
            price.price
        );

        // 保存价格历史 - 使用当前价格作为所有OHLC值
        let today = price.timestamp.date_naive();
        let created_at = price.timestamp.naive_utc();
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO price_history (symbol, date, open_price, high_price, low_price, close_price, volume, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            price.symbol,
            today,
            price.price, // open_price
            price.price, // high_price
            price.price, // low_price
            price.price, // close_price
            price.volume,
            created_at,
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

                    info!(
                        "🔔 Alert {} triggered! {} is now ${:.2} (target: {} ${:.2})",
                        alert_id, symbol, current_price, alert.condition, alert.price
                    );

                    // 获取完整的预警信息并发送邮件通知
                    match self.get_alert_by_id(alert_id).await {
                        Ok(Some(full_alert)) => {
                            info!("Sending email notification for alert {}", alert_id);
                            if let Err(e) = self
                                .email_notifier
                                .send_alert_notification(&full_alert, current_price)
                                .await
                            {
                                error!(
                                    "Failed to send email notification for alert {}: {}",
                                    alert_id, e
                                );
                            } else {
                                info!(
                                    "✅ Email notification sent successfully for alert {}",
                                    alert_id
                                );
                            }
                        }
                        Ok(None) => {
                            error!("Alert {} not found after triggering", alert_id);
                        }
                        Err(e) => {
                            error!(
                                "Failed to fetch alert {} for email notification: {}",
                                alert_id, e
                            );
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
