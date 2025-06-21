use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// 测试股票列表
const TEST_SYMBOLS: &[&str] = &[
    "000001.SZ", // 平安银行
    "000002.SZ", // 万科A
    "600519.SS", // 贵州茅台
    "600036.SS", // 招商银行
    "300015.SZ", // 爱尔眼科
];

#[derive(Debug, Serialize, Deserialize)]
struct StockData {
    symbol: String,
    name: String,
    current_price: f64,
    change_percent: f64,
    volume: u64,
    timestamp: String,
    source: String,
}

#[derive(Debug, Serialize)]
struct ApiTestResult {
    source: String,
    success_rate: f64,
    avg_response_time_ms: u64,
    data_completeness: f64,
    stability_score: f64,
    notes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 A股数据源技术调研 - 开始测试");
    println!("{}", "=".repeat(60));

    let client = Client::new();

    // 测试各个数据源
    let mut results = Vec::new();

    // 1. 新浪财经API
    println!("\n📊 测试新浪财经API...");
    if let Ok(result) = test_sina_api(&client).await {
        results.push(result);
    }

    // 2. 腾讯财经API
    println!("\n📊 测试腾讯财经API...");
    if let Ok(result) = test_tencent_api(&client).await {
        results.push(result);
    }

    // 3. 东方财富API
    println!("\n📊 测试东方财富API...");
    if let Ok(result) = test_eastmoney_api(&client).await {
        results.push(result);
    }

    // 输出综合评估
    println!("\n{}", "=".repeat(60));
    println!("📋 A股数据源对比分析结果");
    println!("{}", "=".repeat(60));

    for result in &results {
        println!("\n🔹 {}", result.source);
        println!("   成功率: {:.1}%", result.success_rate * 100.0);
        println!("   平均响应时间: {}ms", result.avg_response_time_ms);
        println!("   数据完整度: {:.1}%", result.data_completeness * 100.0);
        println!("   稳定性评分: {:.1}/10", result.stability_score * 10.0);
        if !result.notes.is_empty() {
            println!("   备注: {}", result.notes.join(", "));
        }
    }

    // 推荐方案
    if let Some(best) = results.iter().max_by(|a, b| {
        (a.success_rate * 0.4 + a.data_completeness * 0.3 + a.stability_score * 0.3)
            .partial_cmp(
                &(b.success_rate * 0.4 + b.data_completeness * 0.3 + b.stability_score * 0.3),
            )
            .unwrap()
    }) {
        println!("\n🎯 推荐方案: {}", best.source);
        println!(
            "   综合评分: {:.2}",
            best.success_rate * 0.4 + best.data_completeness * 0.3 + best.stability_score * 0.3
        );
    }

    Ok(())
}

// 测试新浪财经API
async fn test_sina_api(client: &Client) -> Result<ApiTestResult> {
    let mut success_count = 0;
    let mut total_response_time = 0u64;
    let mut data_quality_scores = Vec::new();
    let mut notes = Vec::new();

    for symbol in TEST_SYMBOLS {
        let start = Instant::now();

        // 新浪API需要转换股票代码格式
        let sina_symbol = convert_to_sina_format(symbol);
        let url = format!("https://hq.sinajs.cn/list={}", sina_symbol);

        match client
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn")
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let response_time = start.elapsed().as_millis() as u64;
                total_response_time += response_time;

                if response.status().is_success() {
                    let text = response.text().await?;
                    if let Some(data) = parse_sina_response(&text, symbol) {
                        success_count += 1;
                        let quality = evaluate_data_quality(&data);
                        data_quality_scores.push(quality);

                        println!(
                            "  ✅ {} - 价格: ¥{:.2}, 涨跌: {:.2}%, 响应: {}ms",
                            data.name, data.current_price, data.change_percent, response_time
                        );
                    } else {
                        println!("  ❌ {} - 数据解析失败", symbol);
                        notes.push(format!("{}数据解析失败", symbol));
                    }
                } else {
                    println!("  ❌ {} - HTTP错误: {}", symbol, response.status());
                    notes.push(format!("{}HTTP错误", symbol));
                }
            }
            Err(e) => {
                println!("  ❌ {} - 网络错误: {}", symbol, e);
                notes.push(format!("{}网络错误", symbol));
            }
        }

        // 避免请求过于频繁
        sleep(Duration::from_millis(200)).await;
    }

    let success_rate = success_count as f64 / TEST_SYMBOLS.len() as f64;
    let avg_response_time = if success_count > 0 {
        total_response_time / success_count as u64
    } else {
        0
    };
    let data_completeness = if !data_quality_scores.is_empty() {
        data_quality_scores.iter().sum::<f64>() / data_quality_scores.len() as f64
    } else {
        0.0
    };

    // 稳定性评分基于成功率和响应时间
    let stability_score =
        (success_rate * 0.7) + ((1000.0 - avg_response_time.min(1000) as f64) / 1000.0 * 0.3);

    Ok(ApiTestResult {
        source: "新浪财经API".to_string(),
        success_rate,
        avg_response_time_ms: avg_response_time,
        data_completeness,
        stability_score,
        notes,
    })
}

// 测试腾讯财经API
async fn test_tencent_api(client: &Client) -> Result<ApiTestResult> {
    let mut success_count = 0;
    let mut total_response_time = 0u64;
    let mut data_quality_scores = Vec::new();
    let mut notes = Vec::new();

    for symbol in TEST_SYMBOLS {
        let start = Instant::now();

        // 腾讯API格式转换
        let tencent_symbol = convert_to_tencent_format(symbol);
        let url = format!("https://qt.gtimg.cn/q={}", tencent_symbol);

        match client
            .get(&url)
            .header("Referer", "https://stockapp.finance.qq.com")
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let response_time = start.elapsed().as_millis() as u64;
                total_response_time += response_time;

                if response.status().is_success() {
                    let text = response.text().await?;
                    if let Some(data) = parse_tencent_response(&text, symbol) {
                        success_count += 1;
                        let quality = evaluate_data_quality(&data);
                        data_quality_scores.push(quality);

                        println!(
                            "  ✅ {} - 价格: ¥{:.2}, 涨跌: {:.2}%, 响应: {}ms",
                            data.name, data.current_price, data.change_percent, response_time
                        );
                    } else {
                        println!("  ❌ {} - 数据解析失败", symbol);
                        notes.push(format!("{}数据解析失败", symbol));
                    }
                } else {
                    println!("  ❌ {} - HTTP错误: {}", symbol, response.status());
                    notes.push(format!("{}HTTP错误", symbol));
                }
            }
            Err(e) => {
                println!("  ❌ {} - 网络错误: {}", symbol, e);
                notes.push(format!("{}网络错误", symbol));
            }
        }

        sleep(Duration::from_millis(200)).await;
    }

    let success_rate = success_count as f64 / TEST_SYMBOLS.len() as f64;
    let avg_response_time = if success_count > 0 {
        total_response_time / success_count as u64
    } else {
        0
    };
    let data_completeness = if !data_quality_scores.is_empty() {
        data_quality_scores.iter().sum::<f64>() / data_quality_scores.len() as f64
    } else {
        0.0
    };

    let stability_score =
        (success_rate * 0.7) + ((1000.0 - avg_response_time.min(1000) as f64) / 1000.0 * 0.3);

    Ok(ApiTestResult {
        source: "腾讯财经API".to_string(),
        success_rate,
        avg_response_time_ms: avg_response_time,
        data_completeness,
        stability_score,
        notes,
    })
}

// 测试东方财富API
async fn test_eastmoney_api(client: &Client) -> Result<ApiTestResult> {
    let mut success_count = 0;
    let mut total_response_time = 0u64;
    let mut data_quality_scores = Vec::new();
    let mut notes = Vec::new();

    for symbol in TEST_SYMBOLS {
        let start = Instant::now();

        // 东方财富API - 使用股票代码直接查询
        let url = format!("https://push2.eastmoney.com/api/qt/stock/get?secid={}&fields=f43,f44,f45,f46,f47,f48,f57,f58", 
            convert_to_eastmoney_format(symbol));

        match client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let response_time = start.elapsed().as_millis() as u64;
                total_response_time += response_time;

                if response.status().is_success() {
                    let text = response.text().await?;
                    if let Some(data) = parse_eastmoney_response(&text, symbol) {
                        success_count += 1;
                        let quality = evaluate_data_quality(&data);
                        data_quality_scores.push(quality);

                        println!(
                            "  ✅ {} - 价格: ¥{:.2}, 涨跌: {:.2}%, 响应: {}ms",
                            data.name, data.current_price, data.change_percent, response_time
                        );
                    } else {
                        println!("  ❌ {} - 数据解析失败", symbol);
                        notes.push(format!("{}数据解析失败", symbol));
                    }
                } else {
                    println!("  ❌ {} - HTTP错误: {}", symbol, response.status());
                    notes.push(format!("{}HTTP错误", symbol));
                }
            }
            Err(e) => {
                println!("  ❌ {} - 网络错误: {}", symbol, e);
                notes.push(format!("{}网络错误", symbol));
            }
        }

        sleep(Duration::from_millis(200)).await;
    }

    let success_rate = success_count as f64 / TEST_SYMBOLS.len() as f64;
    let avg_response_time = if success_count > 0 {
        total_response_time / success_count as u64
    } else {
        0
    };
    let data_completeness = if !data_quality_scores.is_empty() {
        data_quality_scores.iter().sum::<f64>() / data_quality_scores.len() as f64
    } else {
        0.0
    };

    let stability_score =
        (success_rate * 0.7) + ((1000.0 - avg_response_time.min(1000) as f64) / 1000.0 * 0.3);

    Ok(ApiTestResult {
        source: "东方财富API".to_string(),
        success_rate,
        avg_response_time_ms: avg_response_time,
        data_completeness,
        stability_score,
        notes,
    })
}

// 股票代码格式转换函数
fn convert_to_sina_format(symbol: &str) -> String {
    if symbol.ends_with(".SZ") {
        format!("sz{}", &symbol[..6])
    } else if symbol.ends_with(".SS") {
        format!("sh{}", &symbol[..6])
    } else {
        symbol.to_string()
    }
}

fn convert_to_tencent_format(symbol: &str) -> String {
    if symbol.ends_with(".SZ") {
        format!("sz{}", &symbol[..6])
    } else if symbol.ends_with(".SS") {
        format!("sh{}", &symbol[..6])
    } else {
        symbol.to_string()
    }
}

fn convert_to_eastmoney_format(symbol: &str) -> String {
    if symbol.ends_with(".SZ") {
        format!("0.{}", &symbol[..6])
    } else if symbol.ends_with(".SS") {
        format!("1.{}", &symbol[..6])
    } else {
        symbol.to_string()
    }
}

// 数据解析函数
fn parse_sina_response(text: &str, symbol: &str) -> Option<StockData> {
    // 新浪API返回格式: var hq_str_sz000001="平安银行,27.55,27.25,26.91,27.60,26.20,26.91,26.92,22114263,589824680,..."
    if let Some(start) = text.find('"') {
        if let Some(end) = text.rfind('"') {
            let data_str = &text[start + 1..end];
            let parts: Vec<&str> = data_str.split(',').collect();

            if parts.len() >= 32 {
                let name = parts[0].to_string();
                let current_price: f64 = parts[3].parse().ok()?;
                let prev_close: f64 = parts[2].parse().ok()?;
                let volume: u64 = parts[8].parse().ok()?;

                let change_percent = if prev_close > 0.0 {
                    ((current_price - prev_close) / prev_close) * 100.0
                } else {
                    0.0
                };

                return Some(StockData {
                    symbol: symbol.to_string(),
                    name,
                    current_price,
                    change_percent,
                    volume,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    source: "新浪财经".to_string(),
                });
            }
        }
    }
    None
}

fn parse_tencent_response(text: &str, symbol: &str) -> Option<StockData> {
    // 腾讯API返回格式类似新浪，但字段位置可能不同
    if let Some(start) = text.find('"') {
        if let Some(end) = text.rfind('"') {
            let data_str = &text[start + 1..end];
            let parts: Vec<&str> = data_str.split('~').collect();

            if parts.len() >= 50 {
                let name = parts[1].to_string();
                let current_price: f64 = parts[3].parse().ok()?;
                let change_percent: f64 = parts[32].parse().ok()?;
                let volume: u64 = parts[6].parse::<f64>().ok()? as u64;

                return Some(StockData {
                    symbol: symbol.to_string(),
                    name,
                    current_price,
                    change_percent,
                    volume,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    source: "腾讯财经".to_string(),
                });
            }
        }
    }
    None
}

fn parse_eastmoney_response(text: &str, symbol: &str) -> Option<StockData> {
    // 东方财富API返回JSON格式
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
        if let Some(data) = json.get("data") {
            let name = data.get("f58")?.as_str()?.to_string();
            let current_price = data.get("f43")?.as_f64()?;
            let change_percent = data.get("f170")?.as_f64()?;
            let volume = data.get("f47")?.as_u64()?;

            return Some(StockData {
                symbol: symbol.to_string(),
                name,
                current_price: current_price / 100.0, // 东方财富价格需要除以100
                change_percent: change_percent / 100.0,
                volume,
                timestamp: chrono::Utc::now().to_rfc3339(),
                source: "东方财富".to_string(),
            });
        }
    }
    None
}

// 数据质量评估
fn evaluate_data_quality(data: &StockData) -> f64 {
    let mut score = 0.0;

    // 价格有效性 (0.4权重)
    if data.current_price > 0.0 && data.current_price < 10000.0 {
        score += 0.4;
    }

    // 涨跌幅合理性 (0.3权重)
    if data.change_percent.abs() < 20.0 {
        score += 0.3;
    }

    // 成交量有效性 (0.2权重)
    if data.volume > 0 {
        score += 0.2;
    }

    // 名称有效性 (0.1权重)
    if !data.name.is_empty() && data.name != "N/A" {
        score += 0.1;
    }

    score
}
