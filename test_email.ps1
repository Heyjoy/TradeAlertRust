# 邮件通知功能测试脚本
# 使用方法: .\test_email.ps1

Write-Host "交易预警系统 - 邮件通知测试" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan

# 检查配置文件
if (-not (Test-Path "config.toml")) {
    Write-Host "错误: 找不到 config.toml 配置文件" -ForegroundColor Red
    Write-Host "请确保在项目根目录运行此脚本" -ForegroundColor Yellow
    exit 1
}

# 读取配置文件检查邮件配置
$configContent = Get-Content "config.toml" -Raw
if ($configContent -notmatch '\[email\]') {
    Write-Host "错误: config.toml 中没有找到 [email] 配置段" -ForegroundColor Red
    Write-Host "请参考 docs/email-setup.md 配置邮件设置" -ForegroundColor Yellow
    exit 1
}

Write-Host "✓ 配置文件检查通过" -ForegroundColor Green

# 检查是否需要更新配置
if ($configContent -match 'your_email@gmail\.com') {
    Write-Host "⚠️  发现默认配置，请更新以下配置:" -ForegroundColor Yellow
    Write-Host "  - smtp_username: 您的邮箱地址" -ForegroundColor Yellow
    Write-Host "  - smtp_password: 您的应用专用密码" -ForegroundColor Yellow
    Write-Host "  - from_email: 发件人邮箱" -ForegroundColor Yellow
    Write-Host "  - to_email: 收件人邮箱" -ForegroundColor Yellow
    Write-Host ""
    $continue = Read-Host "是否继续测试? (y/N)"
    if ($continue -ne 'y' -and $continue -ne 'Y') {
        Write-Host "测试已取消" -ForegroundColor Yellow
        exit 0
    }
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
        Write-Host "  1. SMTP配置是否正确" -ForegroundColor Yellow
        Write-Host "  2. 邮箱密码是否为应用专用密码" -ForegroundColor Yellow
        Write-Host "  3. 网络连接是否正常" -ForegroundColor Yellow
    } else {
        Write-Host "✗ 测试邮件发送失败!" -ForegroundColor Red
        Write-Host "$($response.message)" -ForegroundColor Red
        Write-Host ""
        Write-Host "常见解决方案:" -ForegroundColor Yellow
        Write-Host "  1. 检查 config.toml 中的邮件配置" -ForegroundColor Yellow
        Write-Host "  2. 确认使用应用专用密码 (不是账户密码)" -ForegroundColor Yellow
        Write-Host "  3. 检查防火墙和网络设置" -ForegroundColor Yellow
        Write-Host "  4. 查看服务器日志获取详细错误信息" -ForegroundColor Yellow
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

Write-Host "如需帮助，请查看 docs/email-setup.md" -ForegroundColor Blue 