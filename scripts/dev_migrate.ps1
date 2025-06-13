# 开发环境数据库迁移脚本
# 使用方法: .\scripts\dev_migrate.ps1

Write-Host "🔄 开始数据库迁移..." -ForegroundColor Green

# 设置环境变量
$env:DATABASE_URL = "sqlite:data/trade_alert.db"
$env:SQLX_OFFLINE = "false"

# 确保数据目录存在
if (!(Test-Path "data")) {
    New-Item -ItemType Directory -Path "data"
    Write-Host "📁 创建数据目录" -ForegroundColor Yellow
}

# 运行迁移
Write-Host "📊 运行数据库迁移..." -ForegroundColor Blue
try {
    sqlx migrate run
    Write-Host "✅ 数据库迁移完成!" -ForegroundColor Green
} catch {
    Write-Host "❌ 迁移失败: $_" -ForegroundColor Red
    exit 1
}

# 更新 SQLx 查询缓存（可选）
Write-Host "🔄 更新查询缓存..." -ForegroundColor Blue
try {
    cargo sqlx prepare --workspace
    Write-Host "✅ 查询缓存更新完成!" -ForegroundColor Green
} catch {
    Write-Host "⚠️  查询缓存更新失败，但不影响运行: $_" -ForegroundColor Yellow
}

Write-Host "🚀 迁移完成，可以启动应用了!" -ForegroundColor Green
Write-Host "运行命令: cargo run --bin trade_alert_rust" -ForegroundColor Cyan 