use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct SchedulerConfig {
    #[allow(dead_code)]
    pub default_schedule: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PriceFetcherConfig {
    pub update_interval_secs: u64,
    pub cache_ttl_secs: u64,
    pub max_retries: u32,
    pub max_concurrent_requests: usize,
    pub max_requests_per_hour: u64,
    pub request_timeout_secs: u64,
    pub pool_idle_timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    #[serde(default = "default_smtp_server")]
    pub smtp_server: String,
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    #[serde(default = "default_email")]
    pub smtp_username: String,
    #[serde(default = "default_password")]
    pub smtp_password: String,
    #[serde(default = "default_email")]
    pub from_email: String,
    #[serde(default = "default_from_name")]
    pub from_name: String,
    #[serde(default = "default_email")]
    pub to_email: String,
    #[serde(default = "default_email_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DemoConfig {
    pub enabled: bool,
    pub max_alerts_per_user: u32,
    pub data_retention_hours: u64,
    pub disable_email: bool,
    pub show_demo_banner: bool,
    pub rate_limit_per_minute: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    #[allow(dead_code)]
    pub scheduler: SchedulerConfig,
    pub price_fetcher: PriceFetcherConfig,
    pub email: EmailConfig,
    #[serde(default = "default_demo_config")]
    pub demo: DemoConfig,
}

fn default_demo_config() -> DemoConfig {
    DemoConfig {
        enabled: false,
        max_alerts_per_user: 10,
        data_retention_hours: 24,
        disable_email: false,
        show_demo_banner: false,
        rate_limit_per_minute: 30,
    }
}

// EmailConfig默认值函数
fn default_smtp_server() -> String {
    "smtp.gmail.com".to_string()
}

fn default_smtp_port() -> u16 {
    587
}

fn default_email() -> String {
    "change@me.com".to_string()
}

fn default_password() -> String {
    "change_me_password".to_string()
}

fn default_from_name() -> String {
    "TradeAlert".to_string()
}

fn default_email_enabled() -> bool {
    true
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            // 1. 加载本地配置文件（如果存在）
            .add_source(config::File::with_name("config/config.local").required(false))
            // 2. 加载主配置文件（如果存在）
            .add_source(config::File::with_name("config/config").required(false))
            .add_source(config::File::with_name("config/config.toml").required(false))
            // 3. 为了向后兼容，也检查根目录的配置文件
            .add_source(config::File::with_name("config.local").required(false))
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::File::with_name("config.toml").required(false))
            // 4. 环境变量具有最高优先级 - 使用双下划线作为分隔符
            .add_source(config::Environment::with_prefix("TRADE_ALERT").separator("__"))
            .build()?;

        let mut result: Config = config.try_deserialize()?;

        // Railway/Production 环境变量处理
        result.handle_production_env()?;

        // 处理环境变量占位符
        result.resolve_placeholders()?;

        Ok(result)
    }

    fn handle_production_env(&mut self) -> anyhow::Result<()> {
        use std::env;

        // Railway的PORT环境变量支持
        if let Ok(port) = env::var("PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                tracing::info!("使用Railway PORT环境变量: {}", port_num);
                self.server.port = port_num;
                // Railway要求监听0.0.0.0而不是127.0.0.1
                self.server.host = "0.0.0.0".to_string();
            }
        }

        // 如果是生产环境，使用内存数据库作为fallback
        if env::var("RAILWAY_ENVIRONMENT").is_ok() || env::var("PORT").is_ok() {
            // Railway/Production: 强制使用 `data` 目录
            self.database.url = "sqlite:data/trade_alert.db".to_string();
        } else {
            // Local development: 也建议使用 `data` 目录
            if !self.database.url.contains('/') && !self.database.url.contains('\\') {
                self.database.url = "sqlite:data/trade_alert.db".to_string();
            }
        }

        // 演示模式数据库处理
        if self.demo.enabled {
            self.database.url = "sqlite:data/demo_trade_alert.db".to_string();
            tracing::info!("演示模式已启用，使用演示数据库: {}", self.database.url);

            // 演示模式强制禁用邮件
            if self.demo.disable_email {
                self.email.enabled = false;
                tracing::info!("演示模式：邮件通知已禁用");
            }
        }

        Ok(())
    }

    fn resolve_placeholders(&mut self) -> anyhow::Result<()> {
        use std::env;

        // 解析邮件配置中的环境变量占位符
        if self.email.smtp_username.starts_with("${") && self.email.smtp_username.ends_with("}") {
            let var_name = &self.email.smtp_username[2..self.email.smtp_username.len() - 1];
            self.email.smtp_username = env::var(var_name).unwrap_or_else(|_| {
                tracing::warn!("环境变量 {} 未设置，使用默认值", var_name);
                "your_email@gmail.com".to_string()
            });
        }

        if self.email.smtp_password.starts_with("${") && self.email.smtp_password.ends_with("}") {
            let var_name = &self.email.smtp_password[2..self.email.smtp_password.len() - 1];
            self.email.smtp_password = env::var(var_name).unwrap_or_else(|_| {
                tracing::warn!("环境变量 {} 未设置，邮件功能可能无法正常工作", var_name);
                "your_app_password".to_string()
            });
        }

        if self.email.from_email.starts_with("${") && self.email.from_email.ends_with("}") {
            let var_name = &self.email.from_email[2..self.email.from_email.len() - 1];
            self.email.from_email = env::var(var_name).unwrap_or_else(|_| {
                tracing::warn!("环境变量 {} 未设置，使用默认值", var_name);
                "your_email@gmail.com".to_string()
            });
        }

        if self.email.to_email.starts_with("${") && self.email.to_email.ends_with("}") {
            let var_name = &self.email.to_email[2..self.email.to_email.len() - 1];
            self.email.to_email = env::var(var_name).unwrap_or_else(|_| {
                tracing::warn!("环境变量 {} 未设置，使用默认值", var_name);
                "your_email@gmail.com".to_string()
            });
        }

        Ok(())
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address")
    }
}
