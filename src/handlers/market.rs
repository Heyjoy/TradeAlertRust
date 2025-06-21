use axum::{
    extract::{Path, State}, 
    response::{Html, IntoResponse},
    http::StatusCode,
};
use askama::Template;
use crate::services::Database;
use crate::models::Alert;
use std::sync::Arc;

/// 市场类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Market {
    US,     // 美股
    CN,     // A股
    Crypto, // 加密货币
}

impl Market {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "us" => Some(Market::US),
            "cn" => Some(Market::CN),
            "crypto" => Some(Market::Crypto),
            _ => None,
        }
    }

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
pub async fn dashboard_handler() -> impl IntoResponse {
    // TODO: 实现市场数据聚合逻辑
    let markets = vec![
        MarketSummary {
            market: Market::US,
            active_count: 12,
            status: "开盘中".to_string(),
            trend: 1.2f64,
        },
        MarketSummary {
            market: Market::CN,
            active_count: 8,
            status: "休市中".to_string(),
            trend: -0.8f64,
        },
        MarketSummary {
            market: Market::Crypto,
            active_count: 5,
            status: "24h交易".to_string(),
            trend: 3.1f64,
        },
    ];

    let template = DashboardTemplate {
        markets,
        urgent_alerts: vec![], // TODO: 查询紧急预警
        strategies: vec![], // TODO: 查询策略信息
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Failed to render dashboard template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render page").into_response()
        }
    }
}

/// 市场专业页面处理器
pub async fn market_handler(
    Path(market_str): Path<String>,
) -> impl IntoResponse {
    let market = match Market::from_str(&market_str) {
        Some(m) => m,
        None => return (StatusCode::NOT_FOUND, "Market not found").into_response(),
    };

    // TODO: 根据市场类型查询对应的预警
    let alerts = vec![]; // 替换为实际查询逻辑

    let template = MarketTemplate {
        market,
        alerts,
        market_status: "开盘中".to_string(), // TODO: 实时市场状态
        next_event: "6小时后收盘".to_string(), // TODO: 计算下次事件
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Failed to render market template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render page").into_response()
        }
    }
} 