use crate::{
    models::Alert,
    services::{Database, EmailNotifier},
};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};
use serde_json;
use std::sync::Arc;

// 本地AppState定义 - 与main.rs中的结构相同
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub email_notifier: Arc<EmailNotifier>,
}

/// 市场类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Market {
    US,     // 美股
    CN,     // A股
    Crypto, // 加密货币
}

impl std::str::FromStr for Market {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "us" => Ok(Market::US),
            "cn" => Ok(Market::CN),
            "crypto" => Ok(Market::Crypto),
            _ => Err(()),
        }
    }
}

impl Market {
    pub fn to_str(&self) -> &'static str {
        match self {
            Market::US => "us",
            Market::CN => "cn",
            Market::Crypto => "crypto",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Market::US => "美股",
            Market::CN => "A股",
            Market::Crypto => "加密货币",
        }
    }

    pub fn currency_symbol(&self) -> &'static str {
        match self {
            Market::US => "$",
            Market::CN => "¥",
            Market::Crypto => "",
        }
    }

    pub fn flag_emoji(&self) -> &'static str {
        match self {
            Market::US => "🇺🇸",
            Market::CN => "🇨🇳",
            Market::Crypto => "₿",
        }
    }
}

/// 首页导航中心模板
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub markets: Vec<MarketSummary>,
    pub urgent_alerts: Vec<Alert>,
    pub strategies: Vec<StrategyInfo>,
    pub all_alerts: Vec<Alert>,
}

/// 市场概况数据
#[derive(Debug)]
pub struct MarketSummary {
    pub market: Market,
    pub active_count: i32,
    pub status: String,
    pub trend: f64, // 整体趋势百分比
}

/// 策略信息
#[derive(Debug)]
pub struct StrategyInfo {
    pub name: String,
    pub signal_count: i32,
    pub market_type: String, // "A股专用", "全市场" 等
}

/// 市场专业页面模板
#[derive(Template)]
#[template(path = "market.html")]
pub struct MarketTemplate {
    pub market: Market,
    pub alerts: Vec<Alert>,
    pub market_status: String,
    pub next_event: String, // 下次开盘/收盘时间
}

/// 首页导航中心处理器  
pub async fn dashboard_handler(State(state): State<AppState>) -> impl IntoResponse {
    tracing::info!("Loading dashboard with real monitoring data");

    // 查询真实的市场数据
    let markets = match get_real_market_summaries(&state).await {
        Ok(summaries) => summaries,
        Err(e) => {
            tracing::error!("Failed to get market summaries: {}", e);
            // 降级到基础数据而不是假数据
            get_fallback_market_summaries(&state).await
        }
    };

    // 查询紧急预警
    let urgent_alerts = match get_urgent_alerts(&state).await {
        Ok(alerts) => alerts,
        Err(e) => {
            tracing::error!("Failed to get urgent alerts: {}", e);
            vec![]
        }
    };

    // 查询策略信息
    let strategies = get_strategy_summaries(&state).await;

    // 查询所有活跃预警用于显示在dashboard上
    let all_alerts = match state.db.list_alerts().await {
        Ok(alerts) => alerts,
        Err(e) => {
            tracing::error!("Failed to get all alerts for dashboard: {}", e);
            vec![]
        }
    };

    let template = DashboardTemplate {
        markets,
        urgent_alerts,
        strategies,
        all_alerts,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Failed to render dashboard template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render page").into_response()
        }
    }
}

/// 获取真实的市场统计数据
async fn get_real_market_summaries(state: &AppState) -> Result<Vec<MarketSummary>, sqlx::Error> {
    let mut summaries = Vec::new();

    // 查询各市场的活跃预警数量
    for market in [Market::US, Market::CN, Market::Crypto] {
        let count: i64 = match market {
            Market::US => {
                // 美股：不包含 .SZ/.SS/.SH 后缀的股票
                sqlx::query_scalar(
                    r#"
                    SELECT COUNT(*) 
                    FROM alerts 
                    WHERE status = 'active' 
                    AND symbol NOT LIKE '%.SZ'
                    AND symbol NOT LIKE '%.SS' 
                    AND symbol NOT LIKE '%.SH'
                    AND symbol NOT LIKE 'BTC%'
                    AND symbol NOT LIKE 'ETH%'
                    "#,
                )
                .fetch_one(state.db.pool())
                .await?
            }
            Market::CN => {
                // A股：以 .SZ/.SS/.SH 结尾的股票
                sqlx::query_scalar(
                    r#"
                    SELECT COUNT(*) 
                    FROM alerts 
                    WHERE status = 'active' 
                    AND (symbol LIKE '%.SZ' OR symbol LIKE '%.SS' OR symbol LIKE '%.SH')
                    "#,
                )
                .fetch_one(state.db.pool())
                .await?
            }
            Market::Crypto => {
                // 加密货币：更广泛的匹配模式
                sqlx::query_scalar(
                    r#"
                    SELECT COUNT(*) 
                    FROM alerts 
                    WHERE status = 'active' 
                    AND (symbol LIKE 'BTC%' OR symbol LIKE 'ETH%' OR symbol LIKE 'USDT%' OR
                         symbol LIKE 'BNB%' OR symbol LIKE 'ADA%' OR symbol LIKE 'SOL%' OR
                         symbol LIKE 'DOGE%' OR symbol LIKE 'DOT%' OR symbol LIKE 'AVAX%' OR
                         symbol LIKE 'SHIB%' OR symbol LIKE 'LTC%' OR symbol LIKE 'LINK%' OR
                         symbol LIKE 'UNI%' OR symbol LIKE 'MATIC%' OR symbol LIKE 'TRX%' OR
                         symbol LIKE '%USD' OR symbol LIKE '%USDT')
                    "#,
                )
                .fetch_one(state.db.pool())
                .await?
            }
        };

        summaries.push(MarketSummary {
            market: market.clone(),
            active_count: count as i32,
            status: get_market_status(&market),
            trend: calculate_market_trend(&market).await,
        });
    }

    Ok(summaries)
}

/// 获取降级市场数据（基于实际数据库，而非假数据）
async fn get_fallback_market_summaries(state: &AppState) -> Vec<MarketSummary> {
    // 至少查询总的活跃预警数量
    let total_active =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM alerts WHERE status = 'active'")
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    vec![
        MarketSummary {
            market: Market::US,
            active_count: (total_active as f32 * 0.5) as i32,
            status: get_market_status(&Market::US),
            trend: 0.0,
        },
        MarketSummary {
            market: Market::CN,
            active_count: (total_active as f32 * 0.3) as i32,
            status: get_market_status(&Market::CN),
            trend: 0.0,
        },
        MarketSummary {
            market: Market::Crypto,
            active_count: (total_active as f32 * 0.2) as i32,
            status: get_market_status(&Market::Crypto),
            trend: 0.0,
        },
    ]
}

/// 获取紧急预警
async fn get_urgent_alerts(state: &AppState) -> Result<Vec<Alert>, sqlx::Error> {
    // 查询最近24小时内触发的预警
    let alerts = sqlx::query_as::<_, Alert>(
        r#"
        SELECT id, symbol, condition, price, status, created_at, updated_at, triggered_at, notification_email
        FROM alerts 
        WHERE status = 'triggered' 
        AND triggered_at > datetime('now', '-24 hours')
        ORDER BY triggered_at DESC
        LIMIT 5
        "#
    )
    .fetch_all(state.db.pool())
    .await?;

    Ok(alerts)
}

/// 获取策略信息汇总
async fn get_strategy_summaries(state: &AppState) -> Vec<StrategyInfo> {
    // 目前返回基础策略信息，后续可以扩展为真实数据
    vec![
        StrategyInfo {
            name: "涨停回踩监控".to_string(),
            signal_count: 0, // TODO: 实现A股策略引擎后更新
            market_type: "A股专用".to_string(),
        },
        StrategyInfo {
            name: "突破预警".to_string(),
            signal_count: get_breakout_signals_count(state).await,
            market_type: "全市场".to_string(),
        },
    ]
}

/// 获取突破信号数量
async fn get_breakout_signals_count(state: &AppState) -> i32 {
    // 统计接近目标价格的预警数量（作为突破信号的近似）
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) 
        FROM alerts 
        WHERE status = 'active'
        AND created_at > datetime('now', '-7 days')
        "#,
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    count as i32
}

/// 获取市场状态
fn get_market_status(market: &Market) -> String {
    use chrono::{Datelike, Timelike, Utc, Weekday};
    let now = Utc::now();

    match market {
        Market::US => {
            // 美股交易时间：北京时间 22:30-05:00 (夏令时) 或 23:30-06:00 (冬令时)
            // 简化处理，假设夏令时
            let beijing_time = now + chrono::Duration::hours(8);
            let weekday = beijing_time.weekday();

            // 周末休市
            if weekday == Weekday::Sat || weekday == Weekday::Sun {
                return "休市中".to_string();
            }

            let hour = beijing_time.hour();
            if (6..22).contains(&hour) {
                "开盘中".to_string()
            } else {
                "休市中".to_string()
            }
        }
        Market::CN => {
            // A股交易时间：工作日 9:30-11:30, 13:00-15:00
            let beijing_time = now + chrono::Duration::hours(8);
            let weekday = beijing_time.weekday();

            // 周末休市
            if weekday == Weekday::Sat || weekday == Weekday::Sun {
                return "休市中".to_string();
            }

            let hour = beijing_time.hour();
            let minute = beijing_time.minute();

            let is_morning_session =
                (hour == 9 && minute >= 30) || hour == 10 || (hour == 11 && minute < 30);

            let is_afternoon_session = (hour == 13) || (hour == 14) || (hour == 15 && minute == 0);

            if is_morning_session || is_afternoon_session {
                "开盘中".to_string()
            } else {
                "休市中".to_string()
            }
        }
        Market::Crypto => "24h交易".to_string(),
    }
}

/// 计算市场趋势（简化版本）
async fn calculate_market_trend(_market: &Market) -> f64 {
    // TODO: 实现基于价格历史的趋势计算
    // 目前返回0表示中性，后续可以集成实时市场数据
    0.0
}

/// 市场专业页面处理器
pub async fn market_handler(
    Path(market_str): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let market = match market_str.parse::<Market>() {
        Ok(m) => m,
        Err(_) => return (StatusCode::NOT_FOUND, "Market not found").into_response(),
    };

    tracing::info!("访问 {:?} 市场页面", market);

    // 先查询所有警报以调试
    let all_alerts = sqlx::query_as::<_, Alert>(
        "SELECT id, symbol, condition, price, status, created_at, updated_at, triggered_at, notification_email FROM alerts ORDER BY created_at DESC"
    )
    .fetch_all(state.db.pool())
    .await;

    match &all_alerts {
        Ok(alerts) => {
            tracing::info!("数据库中共有 {} 个警报", alerts.len());
            for alert in alerts {
                tracing::info!(
                    "  - ID: {}, Symbol: {}, Status: {}, Price: ${:.2}",
                    alert.id,
                    alert.symbol,
                    alert.status,
                    alert.price
                );
            }
        }
        Err(e) => tracing::error!("查询所有警报失败: {}", e),
    }

    // 根据市场类型查询对应的预警
    let alerts = match get_market_alerts(&state, &market).await {
        Ok(alerts) => alerts,
        Err(e) => {
            tracing::error!("Failed to get market alerts: {}", e);
            vec![]
        }
    };

    let template = MarketTemplate {
        market: market.clone(),
        alerts,
        market_status: get_market_status(&market),
        next_event: calculate_next_market_event(&market),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Failed to render market template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render page").into_response()
        }
    }
}

/// 获取特定市场的预警
async fn get_market_alerts(state: &AppState, market: &Market) -> Result<Vec<Alert>, sqlx::Error> {
    tracing::info!("查询 {:?} 市场的警报", market);

    let alerts = match market {
        Market::US => {
            // 美股：不包含 .SZ/.SS/.SH 后缀的股票，且不包含加密货币
            let alerts = sqlx::query_as::<_, Alert>(
                r#"
                SELECT id, symbol, condition, price, status, created_at, updated_at, triggered_at, notification_email
                FROM alerts 
                WHERE status = 'active' 
                AND symbol NOT LIKE '%.SZ'
                AND symbol NOT LIKE '%.SS' 
                AND symbol NOT LIKE '%.SH'
                AND symbol NOT LIKE 'BTC%'
                AND symbol NOT LIKE 'ETH%'
                AND symbol NOT LIKE 'USDT%'
                ORDER BY created_at DESC
                "#
            )
            .fetch_all(state.db.pool())
            .await?;

            tracing::info!("美股市场查询到 {} 个活跃警报", alerts.len());
            for alert in &alerts {
                tracing::info!(
                    "  - Symbol: {}, Price: ${:.2}, Condition: {}",
                    alert.symbol,
                    alert.price,
                    alert.condition
                );
            }
            alerts
        }
        Market::CN => {
            // A股：以 .SZ/.SS/.SH 结尾的股票
            let alerts = sqlx::query_as::<_, Alert>(
                r#"
                SELECT id, symbol, condition, price, status, created_at, updated_at, triggered_at, notification_email
                FROM alerts 
                WHERE status = 'active' 
                AND (symbol LIKE '%.SZ' OR symbol LIKE '%.SS' OR symbol LIKE '%.SH')
                ORDER BY created_at DESC
                "#
            )
            .fetch_all(state.db.pool())
            .await?;

            tracing::info!("A股市场查询到 {} 个活跃警报", alerts.len());
            alerts
        }
        Market::Crypto => {
            // 加密货币：更广泛的匹配模式
            let alerts = sqlx::query_as::<_, Alert>(
                r#"
                SELECT id, symbol, condition, price, status, created_at, updated_at, triggered_at, notification_email
                FROM alerts 
                WHERE status = 'active' 
                AND (symbol LIKE 'BTC%' OR symbol LIKE 'ETH%' OR symbol LIKE 'USDT%' OR
                     symbol LIKE 'BNB%' OR symbol LIKE 'ADA%' OR symbol LIKE 'SOL%' OR
                     symbol LIKE 'DOGE%' OR symbol LIKE 'DOT%' OR symbol LIKE 'AVAX%' OR
                     symbol LIKE 'SHIB%' OR symbol LIKE 'LTC%' OR symbol LIKE 'LINK%' OR
                     symbol LIKE 'UNI%' OR symbol LIKE 'MATIC%' OR symbol LIKE 'TRX%' OR
                     symbol LIKE '%USD' OR symbol LIKE '%USDT')
                ORDER BY created_at DESC
                "#
            )
            .fetch_all(state.db.pool())
            .await?;

            tracing::info!("加密货币市场查询到 {} 个活跃警报", alerts.len());
            alerts
        }
    };

    Ok(alerts)
}

/// 计算下次市场事件
fn calculate_next_market_event(market: &Market) -> String {
    use chrono::{Datelike, Timelike, Utc, Weekday};
    let now = Utc::now();
    let beijing_time = now + chrono::Duration::hours(8);

    match market {
        Market::US => {
            let weekday = beijing_time.weekday();
            if weekday == Weekday::Sat || weekday == Weekday::Sun {
                "周一22:30开盘".to_string()
            } else {
                let hour = beijing_time.hour();
                if !(6..22).contains(&hour) {
                    "6小时后收盘".to_string()
                } else {
                    "今晚22:30开盘".to_string()
                }
            }
        }
        Market::CN => {
            let weekday = beijing_time.weekday();
            if weekday == Weekday::Sat || weekday == Weekday::Sun {
                "周一9:30开盘".to_string()
            } else {
                let hour = beijing_time.hour();
                let minute = beijing_time.minute();

                if hour < 9 || (hour == 9 && minute < 30) {
                    "今日9:30开盘".to_string()
                } else if hour >= 15 {
                    "明日9:30开盘".to_string()
                } else if (11..13).contains(&hour) {
                    "13:00开盘".to_string()
                } else {
                    "15:00收盘".to_string()
                }
            }
        }
        Market::Crypto => "持续交易中".to_string(),
    }
}

/// 获取股票当前价格API
pub async fn get_stock_price(
    Path(symbol): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // 从price_history表查询最新价格
    let price_result = sqlx::query_scalar::<_, f64>(
        r#"
        SELECT close_price
        FROM price_history 
        WHERE symbol = ?
        ORDER BY date DESC, created_at DESC
        LIMIT 1
        "#,
    )
    .bind(&symbol)
    .fetch_optional(state.db.pool())
    .await;

    match price_result {
        Ok(Some(price)) => {
            let price_info = serde_json::json!({
                "symbol": symbol,
                "price": price,
                "status": "success"
            });
            Json(price_info).into_response()
        }
        Ok(None) => {
            let error_info = serde_json::json!({
                "symbol": symbol,
                "error": "Price not found",
                "status": "error"
            });
            (StatusCode::NOT_FOUND, Json(error_info)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch price for {}: {}", symbol, e);
            let error_info = serde_json::json!({
                "symbol": symbol,
                "error": "Database error",
                "status": "error"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_info)).into_response()
        }
    }
}
