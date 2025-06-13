# 交易预警系统 - 公网启动脚本
Write-Host "🚀 启动交易预警系统（公网版本）" -ForegroundColor Cyan
Write-Host ""

# 检查ngrok是否安装
try {
    ngrok --version | Out-Null
    Write-Host "✓ ngrok 已安装" -ForegroundColor Green
} catch {
    Write-Host "❌ 未找到 ngrok，请先安装 ngrok" -ForegroundColor Red
    Write-Host "下载地址: https://ngrok.com/download" -ForegroundColor Yellow
    Read-Host "按回车键退出"
    exit 1
}

# 检查配置
if (Test-Path "config.local.toml" -or Test-Path ".env") {
    Write-Host "✓ 配置文件已找到" -ForegroundColor Green
} else {
    Write-Host "⚠️ 未找到配置文件，请确保已配置邮箱" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "正在启动服务器..." -ForegroundColor Blue

# 启动服务器
$env:DATABASE_URL = "sqlite:trade_alert.db"
$serverJob = Start-Job -ScriptBlock {
    Set-Location $using:PWD
    cargo run --bin trade_alert_rust
}

# 等待服务器启动
Write-Host "等待服务器启动..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# 检查服务器是否启动成功
try {
    $response = Invoke-WebRequest -Uri "http://localhost:3000" -TimeoutSec 5 -ErrorAction Stop
    Write-Host "✓ 服务器启动成功" -ForegroundColor Green
} catch {
    Write-Host "❌ 服务器启动失败" -ForegroundColor Red
    Stop-Job $serverJob -Force
    Remove-Job $serverJob -Force
    Read-Host "按回车键退出"
    exit 1
}

Write-Host ""
Write-Host "🌐 启动 ngrok 公网映射..." -ForegroundColor Blue

# 启动ngrok
$ngrokJob = Start-Job -ScriptBlock {
    ngrok http 3000
}

# 等待ngrok启动
Start-Sleep -Seconds 5

# 获取ngrok公网地址
Write-Host "📋 获取公网地址..." -ForegroundColor Blue
try {
    $tunnelInfo = Invoke-RestMethod -Uri "http://localhost:4040/api/tunnels" -ErrorAction Stop
    $publicUrl = $tunnelInfo.tunnels | Where-Object { $_.public_url -like "https://*" } | Select-Object -First 1 -ExpandProperty public_url
    
    if ($publicUrl) {
        Write-Host ""
        Write-Host "🎉 公网地址获取成功！" -ForegroundColor Green
        Write-Host ""
        Write-Host "=" * 60 -ForegroundColor Cyan
        Write-Host "📧 发给朋友的地址：$publicUrl" -ForegroundColor Yellow
        Write-Host "=" * 60 -ForegroundColor Cyan
        Write-Host ""
        
        # 复制到剪贴板
        $publicUrl | Set-Clipboard
        Write-Host "✅ 地址已自动复制到剪贴板！" -ForegroundColor Green
        Write-Host ""
        
        Write-Host "🔗 ngrok管理面板: http://localhost:4040" -ForegroundColor Blue
        Write-Host "🖥️  本地访问: http://localhost:3000" -ForegroundColor Blue
    } else {
        Write-Host "⚠️ 未能获取到公网地址，请检查ngrok" -ForegroundColor Yellow
    }
} catch {
    Write-Host "⚠️ 无法连接到ngrok API，请手动检查 http://localhost:4040" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "💡 使用说明：" -ForegroundColor Cyan
Write-Host "   1. 复制上面的公网地址发给朋友" -ForegroundColor White
Write-Host "   2. 朋友可以直接访问您的交易预警系统" -ForegroundColor White
Write-Host "   3. 系统会将邮件发送到朋友填写的邮箱" -ForegroundColor White
Write-Host ""
Write-Host "🛑 按 Ctrl+C 或关闭窗口停止所有服务" -ForegroundColor Red
Write-Host ""

# 保持脚本运行
try {
    while ($true) {
        Start-Sleep -Seconds 30
        
        # 检查服务器是否还在运行
        try {
            Invoke-WebRequest -Uri "http://localhost:3000" -TimeoutSec 3 -ErrorAction Stop | Out-Null
        } catch {
            Write-Host "❌ 服务器已停止" -ForegroundColor Red
            break
        }
    }
} finally {
    # 清理工作
    Write-Host ""
    Write-Host "🧹 正在清理..." -ForegroundColor Yellow
    Stop-Job $serverJob -Force 2>$null
    Remove-Job $serverJob -Force 2>$null
    Stop-Job $ngrokJob -Force 2>$null
    Remove-Job $ngrokJob -Force 2>$null
    
    # 杀死相关进程
    Get-Process -Name "ngrok" -ErrorAction SilentlyContinue | Stop-Process -Force
    
    Write-Host "✅ 清理完成" -ForegroundColor Green
} 