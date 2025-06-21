use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Yahoo Finance API ===");

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let symbols = vec!["AAPL", "GOOGL", "MSFT", "TSLA"];

    for symbol in symbols {
        println!("\nTesting symbol: {}", symbol);

        let url = format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}",
            symbol
        );

        match client.get(&url).send().await {
            Ok(response) => {
                println!("  HTTP Status: {}", response.status());

                match response.json::<YahooQuoteResponse>().await {
                    Ok(data) => {
                        if let Some(error) = data.chart.error {
                            println!(
                                "  ❌ Yahoo API Error: {} - {}",
                                error.code, error.description
                            );
                        } else if data.chart.result.is_empty() {
                            println!("  ❌ No results returned");
                        } else {
                            let result = &data.chart.result[0];
                            if let Some(price) = result.meta.regular_market_price {
                                println!("  ✅ Success!");
                                println!("     Symbol: {}", result.meta.symbol);
                                println!("     Price: ${:.2}", price);
                                if let Some(volume) = result.meta.regular_market_volume {
                                    println!("     Volume: {}", volume);
                                }
                            } else {
                                println!("  ❌ No price data available");
                            }
                        }
                    }
                    Err(e) => {
                        println!("  ❌ JSON parsing error: {}", e);
                        // Print raw response for debugging
                        match client.get(&url).send().await {
                            Ok(raw_response) => {
                                if let Ok(text) = raw_response.text().await {
                                    println!(
                                        "     Raw response (first 200 chars): {}",
                                        text.chars().take(200).collect::<String>()
                                    );
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
            Err(e) => {
                println!("  ❌ Request failed: {}", e);
            }
        }
    }

    println!("\n=== Test Complete ===");
    Ok(())
}
