use anyhow::Result;
use sqlx::sqlite::SqlitePool;
use std::path::Path;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 读取数据库 URL
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:trade_alert.db".to_string());
    
    // 从 URL 中提取数据库文件路径并确保目录和文件存在
    if let Some(db_path_str) = database_url.strip_prefix("sqlite:") {
        let db_path = Path::new(db_path_str);
        // 确保父目录存在
        if let Some(parent_dir) = db_path.parent() {
            if !parent_dir.exists() {
                info!("创建数据库目录: {}", parent_dir.display());
                std::fs::create_dir_all(parent_dir)?;
            }
        }
        // 如果数据库文件不存在，先创建一个空文件
        if !db_path.exists() {
            info!("创建数据库文件: {}", db_path.display());
            std::fs::File::create(db_path)?;
        }
    }
    
    let pool = SqlitePool::connect(&database_url).await?;

    // 读取 migrations 目录下所有 .sql 文件
    let migrations_dir = Path::new("migrations");
    let mut migration_files: Vec<_> = std::fs::read_dir(migrations_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "sql"))
        .collect();
    migration_files.sort_by_key(|entry| entry.path().to_path_buf());

    for entry in migration_files {
        let migration_path = entry.path();
        info!("Running migration: {}", migration_path.display());
        let migration_sql = std::fs::read_to_string(&migration_path)?;
        let statements: Vec<&str> = migration_sql
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        for statement in statements {
            if let Err(e) = sqlx::query(statement).execute(&pool).await {
                error!("Failed to execute migration statement: {}", e);
                return Err(e.into());
            }
        }
    }
    info!("Database migrations completed successfully");
    Ok(())
}
