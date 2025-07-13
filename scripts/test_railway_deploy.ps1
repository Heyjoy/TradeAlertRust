# Railway部署本地模拟脚本
# 模拟Railway环境来排查部署问题

Write-Host "🚀 Railway部署本地模拟测试" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green

# 1. 设置Railway模拟环境变量
Write-Host "`n📝 设置Railway环境变量..." -ForegroundColor Yellow
$env:SQLX_OFFLINE = "true"
$env:TRADE_ALERT_EMAIL_SMTP_SERVER = "smtp.gmail.com"
$env:TRADE_ALERT_EMAIL_SMTP_PORT = "587"
$env:TRADE_ALERT_EMAIL_SMTP_USERNAME = "your-email@gmail.com"
$env:TRADE_ALERT_EMAIL_SMTP_PASSWORD = "your-app-password"
$env:TRADE_ALERT_EMAIL_FROM_EMAIL = "your-email@gmail.com"
$env:TRADE_ALERT_EMAIL_TO_EMAIL = "your-email@gmail.com"
$env:TRADE_ALERT_EMAIL_ENABLED = "true"
$env:TRADE_ALERT_LOGGING_LEVEL = "info"

Write-Host "✅ 环境变量已设置 (使用SQLX_OFFLINE=true)" -ForegroundColor Green

# 2. 生成SQLx离线缓存（如果需要）
Write-Host "`n🗄️ 生成SQLx离线缓存..." -ForegroundColor Yellow
if (Test-Path ".sqlx") {
    Write-Host "⚠️  .sqlx目录已存在，跳过生成" -ForegroundColor Cyan
} else {
    Write-Host "正在生成SQLx缓存文件..."
    $env:DATABASE_URL = "sqlite:data/trade_alert.db"
    $env:SQLX_OFFLINE = "false"
    
    # 确保数据库是最新的
    cargo sqlx migrate run
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ 数据库迁移失败" -ForegroundColor Red
        exit 1
    }
    
    # 生成离线缓存
    cargo sqlx prepare
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ SQLx缓存生成失败" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ SQLx缓存生成完成" -ForegroundColor Green
    $env:SQLX_OFFLINE = "true"
}

# 3. 模拟Railway构建过程
Write-Host "`n🔨 模拟Railway构建过程..." -ForegroundColor Yellow
Write-Host "执行: cargo build --release"

cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 构建失败！这就是Railway部署失败的原因" -ForegroundColor Red
    Write-Host "`n🔍 可能的解决方案:" -ForegroundColor Yellow
    Write-Host "1. 检查SQLx离线模式是否正确设置" -ForegroundColor White
    Write-Host "2. 确保所有数据库迁移都已运行" -ForegroundColor White
    Write-Host "3. 检查代码语法错误" -ForegroundColor White
    exit 1
}

Write-Host "✅ 构建成功！" -ForegroundColor Green

# 4. 测试启动
Write-Host "`n🚀 测试应用启动..." -ForegroundColor Yellow
Write-Host "执行: cargo run --release"
Write-Host "⚠️  按Ctrl+C停止测试" -ForegroundColor Cyan

# 使用timeout命令限制运行时间
$startInfo = New-Object System.Diagnostics.ProcessStartInfo
$startInfo.FileName = "cargo"
$startInfo.Arguments = "run --release"
$startInfo.UseShellExecute = $false
$startInfo.RedirectStandardOutput = $true
$startInfo.RedirectStandardError = $true

$process = New-Object System.Diagnostics.Process
$process.StartInfo = $startInfo

# 设置环境变量
foreach ($env in Get-ChildItem Env:TRADE_ALERT_*) {
    $process.StartInfo.EnvironmentVariables[$env.Name] = $env.Value
}
$process.StartInfo.EnvironmentVariables["SQLX_OFFLINE"] = "true"

Write-Host "启动中..." -ForegroundColor Cyan
$process.Start()

# 读取输出
$timeout = 15  # 15秒超时
$elapsed = 0
$success = $false

while (-not $process.HasExited -and $elapsed -lt $timeout) {
    Start-Sleep -Seconds 1
    $elapsed++
    
    # 检查是否有输出表明服务器启动成功
    if (-not $process.StandardOutput.EndOfStream) {
        $output = $process.StandardOutput.ReadLine()
        Write-Host $output -ForegroundColor Gray
        
        if ($output -match "listening on") {
            $success = $true
            break
        }
    }
}

if ($success) {
    Write-Host "`n✅ 应用启动成功！Railway部署应该可以正常工作" -ForegroundColor Green
} else {
    Write-Host "`n⚠️  应用启动超时或失败" -ForegroundColor Yellow
}

# 停止进程
if (-not $process.HasExited) {
    $process.Kill()
    $process.WaitForExit()
}

Write-Host "`n📋 测试完成!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Green

# 5. 输出Railway部署建议
Write-Host "`n🎯 Railway部署建议:" -ForegroundColor Yellow
Write-Host "1. 确保在Railway中设置 SQLX_OFFLINE=true" -ForegroundColor White
Write-Host "2. 提交 .sqlx/ 目录到Git仓库" -ForegroundColor White
Write-Host "3. 检查railway.toml中的startCommand是否正确" -ForegroundColor White
Write-Host "4. 确保所有必需的环境变量都在Railway中配置" -ForegroundColor White

Write-Host "`n📚 详细故障排除指南: docs/troubleshooting/railway-deployment-issues.md" -ForegroundColor Cyan