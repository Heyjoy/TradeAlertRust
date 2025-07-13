# 交易预警系统启动脚本
Write-Host "🚀 启动交易预警系统..." -ForegroundColor Cyan
Write-Host ""

# 检查配置
if (Test-Path "config.local.toml" -or Test-Path ".env") {
    Write-Host "✓ 配置文件已找到" -ForegroundColor Green
} else {
    Write-Host "⚠️ 未找到配置文件，请确保已配置邮箱" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "正在启动服务器..." -ForegroundColor Blue
cargo run --bin trade_alert_rust 