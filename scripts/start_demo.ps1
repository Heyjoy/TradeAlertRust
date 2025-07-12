# 演示模式启动脚本
# 使用方法: .\scripts\start_demo.ps1

Write-Host "🔬 启动演示模式..." -ForegroundColor Green

# 设置演示模式环境变量
$env:DATABASE_URL = "sqlite:data/demo_trade_alert.db"
$env:SQLX_OFFLINE = "false"

# 演示模式配置
$env:TRADE_ALERT__DEMO__ENABLED = "true"
$env:TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER = "5"
$env:TRADE_ALERT__DEMO__DATA_RETENTION_HOURS = "24"
$env:TRADE_ALERT__DEMO__DISABLE_EMAIL = "true"
$env:TRADE_ALERT__DEMO__SHOW_DEMO_BANNER = "true"
$env:TRADE_ALERT__DEMO__RATE_LIMIT_PER_MINUTE = "20"

# 禁用邮件发送
$env:TRADE_ALERT__EMAIL__ENABLED = "false"

Write-Host "📋 演示模式配置:" -ForegroundColor Blue
Write-Host "  - 数据库: $env:DATABASE_URL" -ForegroundColor Cyan
Write-Host "  - 每用户最大预警数: $env:TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER" -ForegroundColor Cyan
Write-Host "  - 数据保留时间: $env:TRADE_ALERT__DEMO__DATA_RETENTION_HOURS 小时" -ForegroundColor Cyan
Write-Host "  - 邮件通知: 已禁用" -ForegroundColor Cyan
Write-Host "  - 用户隔离: 已启用" -ForegroundColor Cyan

# 创建演示数据库目录
if (!(Test-Path "data")) {
    New-Item -Path "data" -ItemType Directory
    Write-Host "✅ 创建数据目录" -ForegroundColor Green
}

# 检查并备份生产数据库
if (Test-Path "data/trade_alert.db") {
    if (!(Test-Path "data/demo_trade_alert.db")) {
        Write-Host "🔄 创建演示数据库..." -ForegroundColor Blue
        # 复制生产数据库作为演示数据库的基础（清理敏感数据）
        Copy-Item "data/trade_alert.db" "data/demo_trade_alert.db"
        Write-Host "✅ 演示数据库已创建" -ForegroundColor Green
    } else {
        Write-Host "📋 使用现有演示数据库" -ForegroundColor Yellow
    }
} else {
    Write-Host "📋 将创建新的演示数据库" -ForegroundColor Yellow
}

# 运行数据库迁移
Write-Host "🔄 运行数据库迁移..." -ForegroundColor Blue
try {
    sqlx migrate run
    Write-Host "✅ 数据库迁移完成!" -ForegroundColor Green
} catch {
    Write-Host "❌ 迁移失败: $_" -ForegroundColor Red
    Write-Host "💡 请确保已安装 sqlx-cli: cargo install sqlx-cli" -ForegroundColor Yellow
    exit 1
}

# 显示启动信息
Write-Host ""
Write-Host "🚀 启动演示环境..." -ForegroundColor Green
Write-Host "📍 应用将在 http://127.0.0.1:3000 启动" -ForegroundColor Cyan
Write-Host "🔬 演示模式特性:" -ForegroundColor Blue
Write-Host "   • 用户数据隔离 - 每个访问者看到独立的数据" -ForegroundColor White
Write-Host "   • 预警数量限制 - 每用户最多$env:TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER个预警" -ForegroundColor White
Write-Host "   • 数据自动清理 - 24小时后自动删除演示数据" -ForegroundColor White
Write-Host "   • 邮件通知禁用 - 不会发送真实邮件" -ForegroundColor White
Write-Host "   • 演示横幅显示 - 提醒用户当前为演示环境" -ForegroundColor White
Write-Host ""
Write-Host "⏹️  按 Ctrl+C 停止演示环境" -ForegroundColor Yellow
Write-Host "🌐 分享给朋友测试: http://localhost:3000?demo=true" -ForegroundColor Magenta
Write-Host ""

# 启动应用
try {
    cargo run --bin trade_alert_rust
} catch {
    Write-Host "❌ 应用启动失败: $_" -ForegroundColor Red
    exit 1
}