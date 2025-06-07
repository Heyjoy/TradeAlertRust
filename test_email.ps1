# 邮件通知功能测试脚本
# 使用方法: .\test_email.ps1

Write-Host "交易预警系统 - 邮件通知测试" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan

# 检查配置文件
$configFiles = @("config.toml.example", "config.local.toml", "config.toml", ".env")
$foundConfig = $false

foreach ($file in $configFiles) {
    if (Test-Path $file) {
        Write-Host "✓ 找到配置文件: $file" -ForegroundColor Green
        $foundConfig = $true
    }
}

if (-not $foundConfig) {
    Write-Host "错误: 未找到任何配置文件" -ForegroundColor Red
    Write-Host "请参考以下文档进行配置:" -ForegroundColor Yellow
    Write-Host "  - docs/security-config.md (安全配置)" -ForegroundColor Yellow
    Write-Host "  - docs/email-setup.md (邮件设置)" -ForegroundColor Yellow
    exit 1
}

# 检查配置模板
if (-not (Test-Path "config.toml.example")) {
    Write-Host "警告: 配置模板 config.toml.example 不存在" -ForegroundColor Yellow
}

# 检查是否使用了环境变量或本地配置
$usingEnvVars = $false
$usingLocalConfig = $false

if (Test-Path ".env") {
    $envContent = Get-Content ".env" -Raw
    if ($envContent -match "SMTP_USERNAME" -or $envContent -match "TRADE_ALERT_EMAIL") {
        $usingEnvVars = $true
        Write-Host "✓ 检测到 .env 环境变量配置" -ForegroundColor Green
    }
}

if (Test-Path "config.local.toml") {
    $localContent = Get-Content "config.local.toml" -Raw
    if ($localContent -match 'smtp_username.*@') {
        $usingLocalConfig = $true
        Write-Host "✓ 检测到 config.local.toml 本地配置" -ForegroundColor Green
    }
}

# 检查是否仍在使用默认配置
if (Test-Path "config.toml") {
    $configContent = Get-Content "config.toml" -Raw
    if ($configContent -match 'your_email@gmail\.com') {
        Write-Host "⚠️  检测到默认配置，建议使用安全配置方法:" -ForegroundColor Yellow
        Write-Host "  方法1: 创建 .env 文件设置环境变量" -ForegroundColor Yellow
        Write-Host "  方法2: 复制 config.toml.example 为 config.local.toml" -ForegroundColor Yellow
        Write-Host "  方法3: 设置系统环境变量" -ForegroundColor Yellow
        Write-Host ""
        $continue = Read-Host "是否继续测试? (y/N)"
        if ($continue -ne 'y' -and $continue -ne 'Y') {
            Write-Host "测试已取消" -ForegroundColor Yellow
            Write-Host "请查看 docs/security-config.md 了解安全配置方法" -ForegroundColor Blue
            exit 0
        }
    }
}

# 安全提醒
if (-not $usingEnvVars -and -not $usingLocalConfig) {
    Write-Host ""
    Write-Host "🔒 安全提醒:" -ForegroundColor Cyan
    Write-Host "为了保护隐私，建议使用以下安全配置方法之一:" -ForegroundColor Yellow
    Write-Host "1. 环境变量 (.env 文件)" -ForegroundColor Yellow
    Write-Host "2. 本地配置文件 (config.local.toml)" -ForegroundColor Yellow
    Write-Host "3. 系统环境变量" -ForegroundColor Yellow
    Write-Host "详情请查看: docs/security-config.md" -ForegroundColor Blue
    Write-Host ""
}

# 启动服务器检查
Write-Host "检查服务器状态..." -ForegroundColor Blue

try {
    $response = Invoke-WebRequest -Uri "http://localhost:3000" -TimeoutSec 5 -ErrorAction Stop
    Write-Host "✓ 服务器正在运行" -ForegroundColor Green
} catch {
    Write-Host "✗ 服务器未运行，正在启动..." -ForegroundColor Yellow
    
    # 检查是否有cargo
    try {
        cargo --version | Out-Null
        Write-Host "正在启动服务器，请稍候..." -ForegroundColor Blue
        
        # 在后台启动服务器
        $job = Start-Job -ScriptBlock {
            Set-Location $using:PWD
            cargo run
        }
        
        # 等待服务器启动
        Write-Host "等待服务器启动..." -ForegroundColor Blue
        Start-Sleep -Seconds 10
        
        # 再次检查
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:3000" -TimeoutSec 5 -ErrorAction Stop
            Write-Host "✓ 服务器已启动" -ForegroundColor Green
        } catch {
            Write-Host "✗ 服务器启动失败，请手动运行 'cargo run'" -ForegroundColor Red
            Stop-Job $job -Force
            Remove-Job $job -Force
            exit 1
        }
    } catch {
        Write-Host "✗ 未找到 cargo，请先安装 Rust 并运行 'cargo run'" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "发送测试邮件..." -ForegroundColor Blue

try {
    $response = Invoke-RestMethod -Uri "http://localhost:3000/api/test-email" -Method Get -ErrorAction Stop
    
    if ($response.success -eq $true) {
        Write-Host "✓ 测试邮件发送成功!" -ForegroundColor Green
        Write-Host "$($response.message)" -ForegroundColor Green
        Write-Host ""
        Write-Host "请检查您的邮箱 (包括垃圾邮件箱)" -ForegroundColor Yellow
        Write-Host "如果没有收到邮件，请检查:" -ForegroundColor Yellow
        Write-Host "  1. 环境变量或配置文件是否正确设置" -ForegroundColor Yellow
        Write-Host "  2. 邮箱密码是否为应用专用密码" -ForegroundColor Yellow
        Write-Host "  3. 网络连接是否正常" -ForegroundColor Yellow
    } else {
        Write-Host "✗ 测试邮件发送失败!" -ForegroundColor Red
        Write-Host "$($response.message)" -ForegroundColor Red
        Write-Host ""
        Write-Host "常见解决方案:" -ForegroundColor Yellow
        Write-Host "  1. 检查邮件配置是否正确" -ForegroundColor Yellow
        Write-Host "  2. 确认使用应用专用密码 (不是账户密码)" -ForegroundColor Yellow
        Write-Host "  3. 检查防火墙和网络设置" -ForegroundColor Yellow
        Write-Host "  4. 查看服务器日志获取详细错误信息" -ForegroundColor Yellow
        Write-Host "  5. 查看安全配置指南: docs/security-config.md" -ForegroundColor Yellow
    }
} catch {
    Write-Host "✗ 请求失败: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "请确保服务器正在运行并检查网络连接" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "测试完成!" -ForegroundColor Cyan

# 询问是否查看日志
$viewLogs = Read-Host "是否查看服务器日志? (y/N)"
if ($viewLogs -eq 'y' -or $viewLogs -eq 'Y') {
    Write-Host "请查看运行 'cargo run' 的终端窗口获取详细日志" -ForegroundColor Blue
}

Write-Host ""
Write-Host "📚 相关文档:" -ForegroundColor Blue
Write-Host "  - docs/email-setup.md (邮件设置)" -ForegroundColor Blue  
Write-Host "  - docs/security-config.md (安全配置)" -ForegroundColor Blue 