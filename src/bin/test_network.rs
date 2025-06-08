use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== SMTP 网络连接诊断 ===");
    
    // 测试基本网络连接
    println!("\n1. 测试基本网络连接...");
    
    // 测试 Gmail SMTP 服务器连接
    let gmail_servers = vec![
        ("smtp.gmail.com", 587),
        ("smtp.gmail.com", 465),
        ("smtp.gmail.com", 25),
    ];
    
    for (server, port) in gmail_servers {
        print!("   测试连接到 {}:{}... ", server, port);
        
        match timeout(Duration::from_secs(10), TcpStream::connect(&format!("{}:{}", server, port))).await {
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
    
    // 测试其他邮件服务器
    println!("\n2. 测试其他邮件服务器连接...");
    let other_servers = vec![
        ("outlook.office365.com", 587),
        ("smtp.qq.com", 587),
        ("smtp.163.com", 465),
    ];
    
    for (server, port) in other_servers {
        print!("   测试连接到 {}:{}... ", server, port);
        
        match timeout(Duration::from_secs(10), TcpStream::connect(&format!("{}:{}", server, port))).await {
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
    println!("\n3. 测试 DNS 解析...");
    match tokio::net::lookup_host("smtp.gmail.com:587").await {
        Ok(mut addrs) => {
            println!("   Gmail SMTP DNS 解析:");
            while let Some(addr) = addrs.next() {
                println!("     - {}", addr);
            }
        }
        Err(e) => {
            println!("   ❌ DNS 解析失败: {}", e);
        }
    }
    
    // 检查环境信息
    println!("\n4. 环境信息:");
    println!("   操作系统: {}", std::env::consts::OS);
    println!("   架构: {}", std::env::consts::ARCH);
    
    // 建议
    println!("\n=== 问题排查建议 ===");
    println!("1. 检查网络连接是否正常");
    println!("2. 检查防火墙设置是否阻止了 SMTP 连接");
    println!("3. 确认 Gmail 应用专用密码是否正确");
    println!("4. 检查是否启用了两步验证");
    println!("5. 尝试使用 VPN 或其他网络环境");
    
    Ok(())
} 