use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Google 服务连接测试 ===");

    // 测试 Google 相关服务器
    let google_servers = vec![
        ("google.com", 80),
        ("google.com", 443),
        ("gmail.com", 80),
        ("gmail.com", 443),
        ("smtp.gmail.com", 587),
        ("smtp.gmail.com", 465),
        ("smtp.gmail.com", 25),
    ];

    println!("\n正在测试 Google 服务连接...");

    for (server, port) in google_servers {
        print!("   测试连接到 {}:{}... ", server, port);

        match timeout(
            Duration::from_secs(10),
            TcpStream::connect(&format!("{}:{}", server, port)),
        )
        .await
        {
            Ok(Ok(_)) => {
                println!("✅ 连接成功");
            }
            Ok(Err(e)) => {
                println!("❌ 连接失败: {}", e);
            }
            Err(_) => {
                println!("❌ 连接超时");
            }
        }
    }

    // 测试 DNS 解析
    println!("\n=== DNS 解析测试 ===");
    let domains = vec!["google.com", "gmail.com", "smtp.gmail.com"];

    for domain in domains {
        print!("   解析 {}... ", domain);
        match tokio::net::lookup_host(&format!("{}:80", domain)).await {
            Ok(mut addrs) => {
                println!("✅ 解析成功");
                let mut count = 0;
                while let Some(addr) = addrs.next() {
                    if count < 3 {
                        // 只显示前3个IP
                        println!("     - {}", addr.ip());
                        count += 1;
                    }
                }
            }
            Err(e) => {
                println!("❌ 解析失败: {}", e);
            }
        }
    }

    // 测试 HTTP 连接
    println!("\n=== HTTP 连接测试 ===");
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let urls = vec![
        "http://google.com",
        "https://google.com",
        "https://gmail.com",
    ];

    for url in urls {
        print!("   测试 HTTP 请求到 {}... ", url);

        match timeout(Duration::from_secs(10), client.get(url).send()).await {
            Ok(Ok(response)) => {
                println!(
                    "✅ HTTP {} - {}",
                    response.status().as_u16(),
                    response.status().canonical_reason().unwrap_or("OK")
                );
            }
            Ok(Err(e)) => {
                println!("❌ HTTP 请求失败: {}", e);
            }
            Err(_) => {
                println!("❌ HTTP 请求超时");
            }
        }
    }

    println!("\n=== 网络环境信息 ===");

    // 检查环境变量
    if let Ok(proxy) = std::env::var("HTTP_PROXY") {
        println!("   HTTP_PROXY: {}", proxy);
    }
    if let Ok(proxy) = std::env::var("HTTPS_PROXY") {
        println!("   HTTPS_PROXY: {}", proxy);
    }
    if let Ok(proxy) = std::env::var("http_proxy") {
        println!("   http_proxy: {}", proxy);
    }
    if let Ok(proxy) = std::env::var("https_proxy") {
        println!("   https_proxy: {}", proxy);
    }

    println!("\n=== 结论 ===");
    println!("如果上面的测试都成功，说明Gmail SMTP应该也能连接。");
    println!("如果Gmail SMTP仍然失败，可能是：");
    println!("1. VPN没有完全路由SMTP流量");
    println!("2. 应用专用密码需要重新生成");
    println!("3. 需要在Gmail安全设置中允许不够安全的应用");

    Ok(())
}
