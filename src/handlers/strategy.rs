use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

/// 策略信号数据
#[derive(Debug)]
pub struct StrategySignal {
    pub symbol: String,
    pub market: String,      // "🇺🇸", "🇨🇳", "₿"
    pub signal_type: String, // "⚠️ 回踩信号", "✅ 买入时机"
    pub price: f64,
    pub description: String,
    pub generated_at: String,
}

/// 策略监控页面模板
#[derive(Template)]
#[template(path = "strategy.html")]
pub struct StrategyTemplate {
    pub cn_signals: Vec<StrategySignal>,
    pub global_signals: Vec<StrategySignal>,
}

/// 策略监控页面处理器
pub async fn strategy_handler() -> impl IntoResponse {
    // TODO: 查询真实的策略信号数据
    let cn_signals = vec![
        StrategySignal {
            symbol: "000725 京东方A".to_string(),
            market: "🇨🇳".to_string(),
            signal_type: "⚠️ 回踩信号".to_string(),
            price: 4.15,
            description: "昨日涨停，今日-3.2%".to_string(),
            generated_at: "15:20".to_string(),
        },
        StrategySignal {
            symbol: "002415 海康威视".to_string(),
            market: "🇨🇳".to_string(),
            signal_type: "✅ 买入时机".to_string(),
            price: 28.50,
            description: "符合所有买入条件".to_string(),
            generated_at: "14:45".to_string(),
        },
    ];

    let global_signals = vec![StrategySignal {
        symbol: "TSLA".to_string(),
        market: "🇺🇸".to_string(),
        signal_type: "⚠️ 箱体突破".to_string(),
        price: 320.50,
        description: "正在测试阻力位".to_string(),
        generated_at: "09:30".to_string(),
    }];

    let template = StrategyTemplate {
        cn_signals,
        global_signals,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Failed to render strategy template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render page").into_response()
        }
    }
}
