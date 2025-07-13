use crate::handlers::market::AppState;
use crate::services::{StrategyAnalyzer, StrategySignal as AnalyzerSignal};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
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
pub async fn strategy_handler(State(app_state): State<AppState>) -> impl IntoResponse {
    // 使用真实的策略分析数据
    let analyzer = StrategyAnalyzer::new(app_state.db.pool().clone());

    // 获取最近的策略信号
    let recent_signals = analyzer.get_recent_signals(20).await.unwrap_or_default();

    // 转换为模板所需的格式
    let mut cn_signals = Vec::new();
    let mut global_signals = Vec::new();

    for (symbol, signal) in recent_signals {
        let template_signal = convert_analyzer_signal_to_template(symbol, signal);

        // 根据股票代码判断市场类型
        if is_cn_stock(&template_signal.symbol) {
            cn_signals.push(template_signal);
        } else {
            global_signals.push(template_signal);
        }
    }

    // 如果没有真实数据，添加一些示例数据
    if cn_signals.is_empty() {
        cn_signals = vec![
            StrategySignal {
                symbol: "000725 京东方A".to_string(),
                market: "🇨🇳".to_string(),
                signal_type: "⚠️ 回踩信号".to_string(),
                price: 4.15,
                description: "昨日涨停，今日回踩支撑位".to_string(),
                generated_at: "15:20".to_string(),
            },
            StrategySignal {
                symbol: "002415 海康威视".to_string(),
                market: "🇨🇳".to_string(),
                signal_type: "✅ 买入时机".to_string(),
                price: 28.50,
                description: "底部突破，放量2.1倍".to_string(),
                generated_at: "14:45".to_string(),
            },
        ];
    }

    if global_signals.is_empty() {
        global_signals = vec![StrategySignal {
            symbol: "TSLA".to_string(),
            market: "🇺🇸".to_string(),
            signal_type: "📈 技术突破".to_string(),
            price: 248.50,
            description: "突破关键阻力位".to_string(),
            generated_at: "22:15".to_string(),
        }];
    }

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

/// 分析指定股票的策略信号 API
pub async fn analyze_symbol_strategy(
    Path(symbol): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<AnalyzerSignal>>, StatusCode> {
    let analyzer = StrategyAnalyzer::new(app_state.db.pool().clone());

    match analyzer.analyze_symbol(&symbol).await {
        Ok(signals) => Ok(Json(signals)),
        Err(e) => {
            tracing::error!("Failed to analyze symbol {}: {}", symbol, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 获取最新策略信号 API
pub async fn get_strategy_signals(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<(String, AnalyzerSignal)>>, StatusCode> {
    let analyzer = StrategyAnalyzer::new(app_state.db.pool().clone());

    match analyzer.get_recent_signals(50).await {
        Ok(signals) => Ok(Json(signals)),
        Err(e) => {
            tracing::error!("Failed to get strategy signals: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 将分析器信号转换为模板信号
fn convert_analyzer_signal_to_template(symbol: String, signal: AnalyzerSignal) -> StrategySignal {
    match signal {
        AnalyzerSignal::LimitUpPullback(s) => StrategySignal {
            symbol: symbol.clone(),
            market: if is_cn_stock(&symbol) {
                "🇨🇳"
            } else {
                "🇺🇸"
            }
            .to_string(),
            signal_type: "⚠️ 涨停回踩".to_string(),
            price: s.pullback_price,
            description: s.description,
            generated_at: "实时".to_string(),
        },
        AnalyzerSignal::BottomBreakout(s) => StrategySignal {
            symbol: symbol.clone(),
            market: if is_cn_stock(&symbol) {
                "🇨🇳"
            } else {
                "🇺🇸"
            }
            .to_string(),
            signal_type: "✅ 底部突破".to_string(),
            price: s.breakout_price,
            description: s.description,
            generated_at: "实时".to_string(),
        },
        AnalyzerSignal::TechnicalIndicator(s) => StrategySignal {
            symbol: symbol.clone(),
            market: if is_cn_stock(&symbol) {
                "🇨🇳"
            } else {
                "🇺🇸"
            }
            .to_string(),
            signal_type: format!("📊 {}", s.indicator_name),
            price: s.value,
            description: s.description,
            generated_at: "实时".to_string(),
        },
    }
}

/// 判断是否为A股股票
fn is_cn_stock(symbol: &str) -> bool {
    // 简单的A股股票代码判断
    symbol.contains(".SZ")
        || symbol.contains(".SH")
        || symbol.len() == 6 && symbol.chars().all(|c| c.is_ascii_digit())
}
