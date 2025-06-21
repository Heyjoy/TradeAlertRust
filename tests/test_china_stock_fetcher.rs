use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 A股数据获取功能测试");
    println!("{}", "=".repeat(50));

    let client = Client::new();

    // 测试股票列表
    let test_symbols = vec![
        ("000001.SZ", "平安银行"),
        ("000002.SZ", "万科A"),
        ("600519.SS", "贵州茅台"),
        ("600036.SS", "招商银行"),
    ];

    for (symbol, name) in test_symbols {
        println!("\n📊 测试 {} ({})", name, symbol);

        match fetch_china_stock_price(&client, symbol).await {
            Ok((price, volume, stock_name)) => {
                println!(
                    "  ✅ 成功: 价格 ¥{:.2}, 成交量 {}, 名称: {}",
                    price, volume, stock_name
                );
            }
            Err(e) => {
                println!("  ❌ 失败: {}", e);
            }
        }
    }

    println!("\n🎯 测试完成");
    Ok(())
}

// 获取A股价格 - 使用新浪财经API
async fn fetch_china_stock_price(client: &Client, symbol: &str) -> Result<(f64, i64, String)> {
    // 转换股票代码格式
    let sina_symbol = convert_to_sina_format(symbol);
    let url = format!("https://hq.sinajs.cn/list={}", sina_symbol);

    let response = client
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
        return Err(anyhow::anyhow!("HTTP错误: {}", response.status()));
    }

    let text = response.text().await?;
    parse_sina_response(&text, symbol)
}

// 股票代码格式转换
fn convert_to_sina_format(symbol: &str) -> String {
    if symbol.ends_with(".SZ") {
        format!("sz{}", &symbol[..6])
    } else if symbol.ends_with(".SS") {
        format!("sh{}", &symbol[..6])
    } else {
        symbol.to_string()
    }
}

// 解析新浪财经API响应
fn parse_sina_response(text: &str, symbol: &str) -> Result<(f64, i64, String)> {
    // 新浪API返回格式: var hq_str_sz000001="平安银行,27.55,27.25,26.91,27.60,26.20,26.91,26.92,22114263,589824680,..."
    if let Some(start) = text.find('"') {
        if let Some(end) = text.rfind('"') {
            let data_str = &text[start + 1..end];
            let parts: Vec<&str> = data_str.split(',').collect();

            if parts.len() >= 32 {
                let name = parts[0].to_string();
                let current_price: f64 = parts[3]
                    .parse()
                    .map_err(|e| anyhow::anyhow!("价格解析失败: {}", e))?;
                let volume: i64 = parts[8]
                    .parse()
                    .map_err(|e| anyhow::anyhow!("成交量解析失败: {}", e))?;

                return Ok((current_price, volume, name));
            }
        }
    }

    Err(anyhow::anyhow!("无法解析 {} 的数据", symbol))
}
