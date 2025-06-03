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

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub scheduler: SchedulerConfig,
    pub price_fetcher: PriceFetcherConfig,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("TRADE_ALERT"))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address")
    }
} 