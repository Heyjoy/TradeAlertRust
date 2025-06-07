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
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
    pub to_email: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub scheduler: SchedulerConfig,
    pub price_fetcher: PriceFetcherConfig,
    pub email: EmailConfig,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            // 1. 首先加载默认配置模板
            .add_source(config::File::with_name("config.toml.example").required(false))
            // 2. 然后加载本地配置（如果存在）
            .add_source(config::File::with_name("config.local").required(false))
            // 3. 最后加载主配置文件（如果存在）
            .add_source(config::File::with_name("config").required(false))
            // 4. 环境变量具有最高优先级
            .add_source(config::Environment::with_prefix("TRADE_ALERT").separator("_"))
            .build()?;

        let mut result: Config = config.try_deserialize()?;
        
        // 处理环境变量占位符
        result.resolve_placeholders()?;
        
        Ok(result)
    }

    fn resolve_placeholders(&mut self) -> anyhow::Result<()> {
        use std::env;
        
        // 解析邮件配置中的环境变量占位符
        if self.email.smtp_username.starts_with("${") && self.email.smtp_username.ends_with("}") {
            let var_name = &self.email.smtp_username[2..self.email.smtp_username.len()-1];
            self.email.smtp_username = env::var(var_name)
                .unwrap_or_else(|_| {
                    tracing::warn!("环境变量 {} 未设置，使用默认值", var_name);
                    "your_email@gmail.com".to_string()
                });
        }
        
        if self.email.smtp_password.starts_with("${") && self.email.smtp_password.ends_with("}") {
            let var_name = &self.email.smtp_password[2..self.email.smtp_password.len()-1];
            self.email.smtp_password = env::var(var_name)
                .unwrap_or_else(|_| {
                    tracing::warn!("环境变量 {} 未设置，邮件功能可能无法正常工作", var_name);
                    "your_app_password".to_string()
                });
        }
        
        if self.email.from_email.starts_with("${") && self.email.from_email.ends_with("}") {
            let var_name = &self.email.from_email[2..self.email.from_email.len()-1];
            self.email.from_email = env::var(var_name)
                .unwrap_or_else(|_| {
                    tracing::warn!("环境变量 {} 未设置，使用默认值", var_name);
                    "your_email@gmail.com".to_string()
                });
        }
        
        if self.email.to_email.starts_with("${") && self.email.to_email.ends_with("}") {
            let var_name = &self.email.to_email[2..self.email.to_email.len()-1];
            self.email.to_email = env::var(var_name)
                .unwrap_or_else(|_| {
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