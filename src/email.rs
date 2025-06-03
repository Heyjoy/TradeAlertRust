use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use tracing::{error, info, warn};
use crate::config::EmailConfig;
use crate::models::Alert;
use chrono::Local;

pub struct EmailNotifier {
    config: EmailConfig,
    smtp: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailNotifier {
    pub fn new(config: EmailConfig) -> anyhow::Result<Self> {
        if !config.enabled {
            warn!("邮件通知功能已禁用");
        }

        let credentials = Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );

        let smtp = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_server)?
            .port(config.smtp_port)
            .credentials(credentials)
            .build();

        Ok(Self { config, smtp })
    }

    pub async fn send_alert_notification(&self, alert: &Alert, current_price: f64) -> anyhow::Result<()> {
        if !self.config.enabled {
            info!("邮件通知已禁用，跳过发送");
            return Ok(());
        }

        let subject = format!("交易预警触发 - {}", alert.symbol);
        let body = self.create_alert_email_body(alert, current_price)?;

        self.send_email(&subject, &body).await
    }

    pub async fn send_test_email(&self) -> anyhow::Result<()> {
        if !self.config.enabled {
            return Err(anyhow::anyhow!("邮件通知功能已禁用"));
        }

        let subject = "交易预警系统 - 测试邮件";
        let body = self.create_test_email_body();

        self.send_email(&subject, &body).await
    }

    async fn send_email(&self, subject: &str, body: &str) -> anyhow::Result<()> {
        let from_mailbox: Mailbox = format!("{} <{}>", self.config.from_name, self.config.from_email)
            .parse()
            .map_err(|e| anyhow::anyhow!("无效的发件人邮箱格式: {}", e))?;

        let to_mailbox: Mailbox = self.config.to_email
            .parse()
            .map_err(|e| anyhow::anyhow!("无效的收件人邮箱格式: {}", e))?;

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .map_err(|e| anyhow::anyhow!("构建邮件失败: {}", e))?;

        match self.smtp.send(email).await {
            Ok(_) => {
                info!("邮件发送成功: {}", subject);
                Ok(())
            }
            Err(e) => {
                error!("邮件发送失败: {}", e);
                Err(anyhow::anyhow!("邮件发送失败: {}", e))
            }
        }
    }

    fn create_alert_email_body(&self, alert: &Alert, current_price: f64) -> anyhow::Result<String> {
        let now = Local::now();
        let alert_type = match alert.condition {
            crate::models::AlertCondition::Above => "突破上限",
            crate::models::AlertCondition::Below => "跌破下限",
        };

        let price_change = if alert.condition == crate::models::AlertCondition::Above {
            format!("价格从 ${:.4} 上涨至 ${:.4}", alert.price, current_price)
        } else {
            format!("价格从 ${:.4} 下跌至 ${:.4}", alert.price, current_price)
        };

        let body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; text-align: center; }}
        .alert-box {{ background-color: #fff3cd; border: 1px solid #ffeaa7; padding: 15px; border-radius: 5px; margin: 20px 0; }}
        .details {{ background-color: #f8f9fa; padding: 15px; border-radius: 5px; margin: 10px 0; }}
        .footer {{ text-align: center; color: #666; font-size: 12px; margin-top: 30px; }}
        .price {{ font-size: 24px; font-weight: bold; color: #e74c3c; }}
        .symbol {{ font-size: 20px; font-weight: bold; color: #2c3e50; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚨 交易预警触发</h1>
            <div class="symbol">{}</div>
        </div>
        
        <div class="alert-box">
            <h2>预警详情</h2>
            <p><strong>预警类型:</strong> {}</p>
            <p><strong>触发价格:</strong> <span class="price">${:.4}</span></p>
            <p><strong>设定价格:</strong> ${:.4}</p>
            <p><strong>触发时间:</strong> {}</p>
        </div>
        
        <div class="details">
            <h3>价格变化</h3>
            <p>{}</p>
            
            <h3>预警信息</h3>
            <p><strong>预警ID:</strong> {}</p>
            <p><strong>创建时间:</strong> {}</p>
            <p><strong>状态:</strong> {}</p>
        </div>
        
        <div class="footer">
            <p>此邮件由交易预警系统自动发送</p>
            <p>发送时间: {}</p>
        </div>
    </div>
</body>
</html>
            "#,
            alert.symbol,
            alert_type,
            current_price,
            alert.price,
            now.format("%Y-%m-%d %H:%M:%S"),
            price_change,
            alert.id,
            alert.created_at.format("%Y-%m-%d %H:%M:%S"),
            alert.status,
            now.format("%Y-%m-%d %H:%M:%S")
        );

        Ok(body)
    }

    fn create_test_email_body(&self) -> String {
        let now = Local::now();
        
        format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background-color: #d4edda; padding: 20px; border-radius: 5px; text-align: center; }}
        .content {{ padding: 20px; background-color: #f8f9fa; border-radius: 5px; margin: 20px 0; }}
        .footer {{ text-align: center; color: #666; font-size: 12px; margin-top: 30px; }}
        .success {{ color: #155724; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1 class="success">✅ 邮件系统测试成功</h1>
        </div>
        
        <div class="content">
            <h2>系统状态</h2>
            <p>恭喜！您的交易预警系统邮件功能已成功配置并可以正常发送邮件。</p>
            
            <h3>配置信息</h3>
            <ul>
                <li><strong>SMTP服务器:</strong> {}</li>
                <li><strong>发件人:</strong> {}</li>
                <li><strong>收件人:</strong> {}</li>
                <li><strong>发送时间:</strong> {}</li>
            </ul>
            
            <h3>下一步</h3>
            <p>现在您可以设置交易预警，当价格触发条件时，系统会自动发送邮件通知您。</p>
        </div>
        
        <div class="footer">
            <p>此邮件由交易预警系统自动发送</p>
        </div>
    </div>
</body>
</html>
            "#,
            self.config.smtp_server,
            self.config.from_email,
            self.config.to_email,
            now.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_body_creation() {
        let config = EmailConfig {
            smtp_server: "smtp.test.com".to_string(),
            smtp_port: 587,
            smtp_username: "test@test.com".to_string(),
            smtp_password: "password".to_string(),
            from_email: "test@test.com".to_string(),
            from_name: "Test System".to_string(),
            to_email: "user@test.com".to_string(),
            enabled: false,
        };

        let notifier = EmailNotifier::new(config).unwrap();
        let test_body = notifier.create_test_email_body();
        
        assert!(test_body.contains("邮件系统测试成功"));
        assert!(test_body.contains("smtp.test.com"));
    }
} 