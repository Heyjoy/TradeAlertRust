use config;
use trade_alert_rust::{config::Config, services::EmailNotifier};

#[tokio::test]
async fn test_email_integration() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt().with_test_writer().try_init().ok(); // 忽略重复初始化错误

    // 加载测试配置
    dotenvy::dotenv().ok();

    // 使用测试配置文件
    let config = config::Config::builder()
        .add_source(config::File::with_name("tests/test_config"))
        .build()?;

    let config: Config = config.try_deserialize()?;

    println!("邮件配置:");
    println!("SMTP服务器: {}", config.email.smtp_server);
    println!("SMTP端口: {}", config.email.smtp_port);
    println!(
        "用户名: {}***",
        &config
            .email
            .smtp_username
            .chars()
            .take(3)
            .collect::<String>()
    );
    println!(
        "发件人: {}***",
        &config.email.from_email.chars().take(3).collect::<String>()
    );
    println!(
        "收件人: {}***",
        &config.email.to_email.chars().take(3).collect::<String>()
    );
    println!("是否启用: {}", config.email.enabled);

    // 创建邮件通知器
    println!("\n创建邮件通知器...");
    let email_notifier = EmailNotifier::new(config.email)?;

    // 发送测试邮件
    println!("发送测试邮件...");
    match email_notifier.send_test_email().await {
        Ok(_) => println!("✅ 测试邮件发送成功！"),
        Err(e) => {
            println!("❌ 测试邮件发送失败: {}", e);
            println!("详细错误: {:?}", e);
        }
    }

    Ok(())
}
