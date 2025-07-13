# 开发环境启动脚本
# 使用方法: .\scripts\dev_start.ps1

Write-Host "🚀 启动开发环境..." -ForegroundColor Green

# 设置环境变量
$env:DATABASE_URL = "sqlite:data/trade_alert.db"
$env:SQLX_OFFLINE = "false"

# 检查并运行迁移
Write-Host "🔄 检查数据库迁移..." -ForegroundColor Blue
try {
    sqlx migrate run
    Write-Host "✅ 数据库迁移完成!" -ForegroundColor Green
} catch {
    Write-Host "❌ 迁移失败: $_" -ForegroundColor Red
    Write-Host "💡 尝试运行: .\scripts\dev_migrate.ps1" -ForegroundColor Yellow
    exit 1
}

# 启动应用
Write-Host "🎯 启动应用..." -ForegroundColor Blue
Write-Host "📍 应用将在 http://127.0.0.1:3000 启动" -ForegroundColor Cyan
Write-Host "⏹️  按 Ctrl+C 停止应用" -ForegroundColor Yellow
Write-Host ""

try {
    cargo run --bin trade_alert_rust
} catch {
    Write-Host "❌ 应用启动失败: $_" -ForegroundColor Red
    exit 1
} 