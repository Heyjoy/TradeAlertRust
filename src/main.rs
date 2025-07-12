mod config;
mod models;
mod services;
mod templates;
mod handlers;

use crate::models::{AlertResponse, CreateAlertRequest};
use crate::services::{Database, EmailNotifier, PriceService};
use crate::templates::{AlertFormTemplate, IndexTemplate};
use crate::handlers::{market::{dashboard_handler, market_handler, AppState}, strategy_handler};
use askama::Template;
use axum::{
    extract::{Json, Path, State, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 使用handlers模块中的AppState定义

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载.env文件（如果存在）- 必须在配置加载之前
    dotenvy::dotenv().ok();
    println!(
        "Loaded env vars: {:?}",
        std::env::vars().collect::<Vec<_>>()
    );

    // Load configuration
    let config = config::Config::load()?;

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.logging.level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db = Arc::new(Database::new(&config.database.url).await?);

    // Initialize email notifier
    let email_notifier = Arc::new(EmailNotifier::new(config.email.clone())?);

    // Initialize price service with Arc
    let price_service = Arc::new(PriceService::new(
        db.pool().clone(),
        &config.price_fetcher,
        email_notifier.clone(),
    ));
    let price_config = Arc::new(config.price_fetcher.clone());
    price_service.start_price_updater(price_config).await;

    // Create application state
    let state = AppState {
        db: db.clone(),
        email_notifier,
    };

    // Build our application with a route
    let app = Router::new()
        // 多市场UI路由
        .route("/", get(dashboard_handler))
        .route("/market/:market", get(market_handler))
        .route("/strategy", get(strategy_handler))
        // 原有路由保持兼容
        .route("/alerts", get(index_page))
        .route("/alerts/new", get(new_alert_form))
        .route("/alerts/:id/edit", get(edit_alert_form))
        .route("/api/alerts", get(list_alerts).post(create_alert))
        .route(
            "/api/alerts/:id",
            get(get_alert).delete(delete_alert).put(update_alert),
        )
        .route("/api/prices/:symbol", get(get_price_history))
        .route("/api/prices/:symbol/latest", get(get_latest_price))
        .route("/api/prices/:symbol/history", get(get_price_history))
        .route("/api/test-email", get(send_test_email))
        // 新增股票搜索API
        .route("/api/stocks/search", get(search_stocks))
        .route("/api/stocks/markets", get(get_markets))
        // 股票价格API
        .route("/api/stock-price/:symbol", get(handlers::market::get_stock_price))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Run it
    let addr = config.server_addr();
    tracing::info!("listening on {}", addr);

    // Use tokio's Server instead of axum's
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// 首页处理函数
async fn index_page(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_alerts().await {
        Ok(alerts) => {
            let template = IndexTemplate::new(alerts);
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    tracing::error!("Failed to render template: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render template",
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to list alerts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list alerts").into_response()
        }
    }
}

// API handlers
async fn create_alert(
    State(state): State<AppState>,
    Json(payload): Json<CreateAlertRequest>,
) -> impl IntoResponse {
    match state.db.create_alert(&payload).await {
        Ok(alert) => (StatusCode::CREATED, Json(AlertResponse::from(alert))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create alert").into_response()
        }
    }
}

async fn list_alerts(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.list_alerts().await {
        Ok(alerts) => Json(
            alerts
                .into_iter()
                .map(AlertResponse::from)
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to list alerts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list alerts").into_response()
        }
    }
}

async fn get_alert(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match state.db.get_alert(id).await {
        Ok(Some(alert)) => Json(AlertResponse::from(alert)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to get alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get alert").into_response()
        }
    }
}

async fn delete_alert(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match state.db.delete_alert(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to delete alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete alert").into_response()
        }
    }
}

async fn update_alert(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateAlertRequest>,
) -> impl IntoResponse {
    match state.db.update_alert(id, &payload).await {
        Ok(Some(alert)) => Json(AlertResponse::from(alert)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to update alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update alert").into_response()
        }
    }
}

// 新建预警表单
async fn new_alert_form() -> impl IntoResponse {
    let template = AlertFormTemplate::new(None);
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Failed to render template: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response()
        }
    }
}

// 编辑预警表单
async fn edit_alert_form(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match state.db.get_alert(id).await {
        Ok(Some(alert)) => {
            let template = AlertFormTemplate::new(Some(alert));
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    tracing::error!("Failed to render template: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render template",
                    )
                        .into_response()
                }
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Alert not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to get alert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get alert").into_response()
        }
    }
}

// 获取股票价格历史
async fn get_price_history(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        SELECT close_price as price, volume, date, created_at
        FROM price_history
        WHERE symbol = ?
        ORDER BY date DESC
        LIMIT 100
        "#,
        symbol
    )
    .fetch_all(state.db.pool())
    .await;

    match result {
        Ok(prices) => {
            let price_data: Vec<_> = prices
                .into_iter()
                .map(|row| {
                    serde_json::json!({
                        "price": row.price,
                        "volume": row.volume,
                        "date": row.date,
                        "created_at": row.created_at
                    })
                })
                .collect();

            Json(serde_json::json!({
                "symbol": symbol,
                "prices": price_data
            }))
            .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get price history for {}: {}", symbol, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get price history",
            )
                .into_response()
        }
    }
}

/// 检测股票代码所属市场并返回货币信息
fn detect_market_info(symbol: &str) -> (String, String, String) {
    // 返回 (market, currency, currency_symbol)
    if symbol.ends_with(".SZ") || symbol.ends_with(".SS") || symbol.ends_with(".SH") {
        // A股市场
        ("cn".to_string(), "CNY".to_string(), "¥".to_string())
    } else if symbol.contains("BTC") || symbol.contains("ETH") || symbol.contains("USDT") {
        // 加密货币市场
        ("crypto".to_string(), "USDT".to_string(), "".to_string())
    } else {
        // 默认美股市场
        ("us".to_string(), "USD".to_string(), "$".to_string())
    }
}

// 获取股票最新价格
async fn get_latest_price(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> impl IntoResponse {
    let (market, currency, currency_symbol) = detect_market_info(&symbol);
    
    let result = sqlx::query!(
        r#"
        SELECT close_price as price, volume, date, created_at
        FROM price_history
        WHERE symbol = ?
        ORDER BY date DESC
        LIMIT 1
        "#,
        symbol
    )
    .fetch_optional(state.db.pool())
    .await;

    match result {
        Ok(Some(row)) => Json(serde_json::json!({
            "symbol": symbol,
            "price": row.price,
            "volume": row.volume,
            "date": row.date,
            "created_at": row.created_at,
            "market": market,
            "currency": currency,
            "currency_symbol": currency_symbol,
            "name_en": null // 数据库无公司名，返回null
        }))
        .into_response(),
        Ok(None) => {
            // 数据库没有，实时查Yahoo
            match fetch_price_from_yahoo(&symbol).await {
                Ok((price, volume, name_en)) => {
                    let today = chrono::Utc::now().date_naive();
                    let now = chrono::Utc::now().naive_utc();
                    // 写入数据库 - 使用当前价格作为所有OHLC值
                    let _ = sqlx::query!(
                        r#"
                        INSERT OR REPLACE INTO price_history (symbol, date, open_price, high_price, low_price, close_price, volume, created_at)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                        "#,
                        symbol,
                        today,
                        price, // open_price
                        price, // high_price  
                        price, // low_price
                        price, // close_price
                        volume,
                        now,
                    )
                    .execute(state.db.pool())
                    .await;
                    Json(serde_json::json!({
                        "symbol": symbol,
                        "price": price,
                        "volume": volume,
                        "date": today,
                        "created_at": now,
                        "market": market,
                        "currency": currency,
                        "currency_symbol": currency_symbol,
                        "name_en": name_en
                    }))
                    .into_response()
                }
                Err(e) => {
                    tracing::error!("Failed to fetch price from Yahoo for {}: {}", symbol, e);
                    (StatusCode::NOT_FOUND, "Unable to fetch current price").into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to get latest price for {}: {}", symbol, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get latest price",
            )
                .into_response()
        }
    }
}

// 辅助函数：直接从Yahoo API获取价格和公司名
async fn fetch_price_from_yahoo(symbol: &str) -> anyhow::Result<(f64, i64, Option<String>)> {
    use serde::Deserialize;
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
        #[serde(rename = "shortName")]
        short_name: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct YahooError {
        code: String,
        description: String,
    }
    let client = reqwest::Client::new();
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}",
        symbol
    );
    let response = client
        .get(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .timeout(std::time::Duration::from_secs(10))
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
    let name_en = result.meta.short_name;
    Ok((price, volume, name_en))
}

async fn send_test_email(State(state): State<AppState>) -> impl IntoResponse {
    match state.email_notifier.send_test_email().await {
        Ok(_) => {
            tracing::info!("测试邮件发送成功");
            Json(serde_json::json!({
                "success": true,
                "message": "测试邮件发送成功，请检查您的邮箱"
            }))
            .into_response()
        }
        Err(e) => {
            tracing::error!("测试邮件发送失败: {}", e);
            Json(serde_json::json!({
                "success": false,
                "message": format!("测试邮件发送失败: {}", e)
            }))
            .into_response()
        }
    }
}

/// 股票搜索API
async fn search_stocks(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let query = params.get("q").map(|s| s.as_str()).unwrap_or("");
    let market = params.get("market").map(|s| s.as_str()).unwrap_or("all");
    
    if query.is_empty() {
        return Json(serde_json::json!({
            "results": [],
            "message": "请输入搜索关键词"
        })).into_response();
    }
    
    let mut results = Vec::new();
    
    // 搜索A股
    if market == "all" || market == "cn" {
        if let Ok(cn_results) = search_cn_stocks(&state, query).await {
            results.extend(cn_results);
        }
    }
    
    // 搜索美股
    if market == "all" || market == "us" {
        if let Ok(us_results) = search_us_stocks(&state, query).await {
            results.extend(us_results);
        }
    }
    
    // 限制返回结果数量
    results.truncate(10);
    
    Json(serde_json::json!({
        "results": results,
        "total": results.len()
    })).into_response()
}

/// 获取支持的市场列表
async fn get_markets() -> impl IntoResponse {
    Json(serde_json::json!({
        "markets": [
            {
                "code": "us",
                "name": "美股",
                "name_en": "US Stocks",
                "symbol_format": "AAPL",
                "currency": "USD",
                "currency_symbol": "$"
            },
            {
                "code": "cn", 
                "name": "A股",
                "name_en": "China A-Shares",
                "symbol_format": "000001.SZ",
                "currency": "CNY",
                "currency_symbol": "¥"
            },
            {
                "code": "crypto",
                "name": "加密货币", 
                "name_en": "Cryptocurrency",
                "symbol_format": "BTC",
                "currency": "USDT",
                "currency_symbol": ""
            }
        ]
    })).into_response()
}

/// 搜索A股股票
async fn search_cn_stocks(state: &AppState, query: &str) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let search_pattern = format!("%{}%", query);
    
    let results = sqlx::query!(
        r#"
        SELECT symbol, code, exchange, name_cn, name_en, pinyin, pinyin_short, industry
        FROM cn_stocks 
        WHERE status = 'active' 
        AND (
            symbol LIKE ?1 OR 
            code LIKE ?1 OR 
            name_cn LIKE ?1 OR 
            pinyin LIKE ?1 OR 
            pinyin_short LIKE ?1
        )
        ORDER BY 
            CASE 
                WHEN symbol = ?2 THEN 1
                WHEN code = ?2 THEN 2
                WHEN name_cn = ?2 THEN 3
                WHEN symbol LIKE ?1 THEN 4
                WHEN code LIKE ?1 THEN 5
                WHEN name_cn LIKE ?1 THEN 6
                ELSE 7
            END
        LIMIT 5
        "#,
        search_pattern,
        query
    )
    .fetch_all(state.db.pool())
    .await?;
    
    let stocks: Vec<serde_json::Value> = results
        .into_iter()
        .map(|row| {
            serde_json::json!({
                "symbol": row.symbol,
                "code": row.code,
                "exchange": row.exchange,
                "name_cn": row.name_cn,
                "name_en": row.name_en,
                "pinyin": row.pinyin,
                "pinyin_short": row.pinyin_short,
                "industry": row.industry,
                "market": "cn",
                "display_name": format!("{} ({})", row.name_cn, row.symbol),
                "search_text": format!("{} {} {} {}", row.symbol, row.name_cn, row.pinyin, row.pinyin_short)
            })
        })
        .collect();
    
    Ok(stocks)
}

/// 搜索美股股票
async fn search_us_stocks(state: &AppState, query: &str) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let search_pattern = format!("%{}%", query.to_uppercase());
    let search_pattern_cn = format!("%{}%", query);
    
    let results = sqlx::query!(
        r#"
        SELECT symbol, name_en, name_cn, sector, exchange
        FROM us_stocks 
        WHERE status = 'active' 
        AND (
            UPPER(symbol) LIKE ?1 OR 
            UPPER(name_en) LIKE ?1 OR 
            name_cn LIKE ?2
        )
        ORDER BY 
            CASE 
                WHEN UPPER(symbol) = UPPER(?3) THEN 1
                WHEN UPPER(symbol) LIKE ?1 THEN 2
                WHEN UPPER(name_en) LIKE ?1 THEN 3
                ELSE 4
            END
        LIMIT 5
        "#,
        search_pattern,
        search_pattern_cn,
        query
    )
    .fetch_all(state.db.pool())
    .await?;
    
    let stocks: Vec<serde_json::Value> = results
        .into_iter()
        .map(|row| {
            let display_name = if let Some(name_cn) = &row.name_cn {
                format!("{} {} ({})", row.symbol, name_cn, row.name_en)
            } else {
                format!("{} ({})", row.symbol, row.name_en)
            };
            
            serde_json::json!({
                "symbol": row.symbol,
                "name_en": row.name_en,
                "name_cn": row.name_cn,
                "sector": row.sector,
                "exchange": row.exchange,
                "market": "us",
                "display_name": display_name,
                "search_text": format!("{} {} {}", row.symbol, row.name_en, row.name_cn.unwrap_or_default())
            })
        })
        .collect();
    
    Ok(stocks)
}
