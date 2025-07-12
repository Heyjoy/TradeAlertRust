#!/usr/bin/env pwsh
# TradeAlert 启动诊断和自动修复脚本

Write-Host "🔍 TradeAlert 启动诊断工具 v2.1" -ForegroundColor Cyan
Write-Host "=" * 50

$hasIssues = $false

# 1. 检查端口占用
Write-Host "`n📡 检查端口占用状态..." -ForegroundColor Yellow
$port3000 = netstat -ano | findstr :3000
if ($port3000) {
    Write-Host "❌ 端口3000被占用:" -ForegroundColor Red
    Write-Host $port3000
    
    # 自动杀死占用进程
    Write-Host "`n🔧 正在自动解决端口占用问题..." -ForegroundColor Yellow
    
    # 杀死TradeAlert相关进程
    $killed = $false
    try {
        $processes = taskkill /f /im trade_alert_rust.exe 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ 已杀死 trade_alert_rust.exe 进程" -ForegroundColor Green
            $killed = $true
        }
    } catch {
        Write-Host "⚠️  未找到 trade_alert_rust.exe 进程" -ForegroundColor Yellow
    }
    
    try {
        $processes = taskkill /f /im cargo.exe 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ 已杀死 cargo.exe 进程" -ForegroundColor Green
            $killed = $true
        }
    } catch {
        Write-Host "⚠️  未找到 cargo.exe 进程" -ForegroundColor Yellow
    }
    
    # 再次检查端口
    Start-Sleep -Seconds 2
    $port3000After = netstat -ano | findstr :3000
    if (-not $port3000After) {
        Write-Host "✅ 端口3000现已可用" -ForegroundColor Green
    } else {
        Write-Host "⚠️  端口仍被占用，建议手动处理或重启系统" -ForegroundColor Yellow
        $hasIssues = $true
    }
} else {
    Write-Host "✅ 端口3000可用" -ForegroundColor Green
}

# 2. 检查环境变量
Write-Host "`n🔧 检查环境变量配置..." -ForegroundColor Yellow
$envVars = @{
    "SQLX_OFFLINE" = "false"
    "DATABASE_URL" = "sqlite:data/trade_alert.db"
    "RUST_LOG" = "info"
}

foreach ($var in $envVars.Keys) {
    $currentValue = [Environment]::GetEnvironmentVariable($var)
    $expectedValue = $envVars[$var]
    
    if ($currentValue -eq $expectedValue) {
        Write-Host "✅ $var = $currentValue" -ForegroundColor Green
    } elseif ($currentValue) {
        Write-Host "⚠️  $var = $currentValue (期望: $expectedValue)" -ForegroundColor Yellow
    } else {
        Write-Host "❌ $var 未设置，正在设置为: $expectedValue" -ForegroundColor Red
        [Environment]::SetEnvironmentVariable($var, $expectedValue)
        Write-Host "✅ 已设置 $var = $expectedValue" -ForegroundColor Green
    }
}

# 3. 检查数据库文件
Write-Host "`n💾 检查数据库状态..." -ForegroundColor Yellow
if (Test-Path "data") {
    Write-Host "✅ 数据目录存在" -ForegroundColor Green
} else {
    Write-Host "❌ 数据目录不存在，正在创建..." -ForegroundColor Red
    New-Item -ItemType Directory -Path "data" -Force
    Write-Host "✅ 已创建数据目录" -ForegroundColor Green
}

if (Test-Path "data/trade_alert.db") {
    Write-Host "✅ 数据库文件存在" -ForegroundColor Green
    
    # 检查数据库表
    try {
        $tables = sqlite3 data/trade_alert.db ".tables" 2>$null
        if ($tables) {
            Write-Host "✅ 数据库表结构正常" -ForegroundColor Green
        } else {
            Write-Host "⚠️  数据库表可能为空，建议运行迁移" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "⚠️  无法检查数据库内容（可能缺少sqlite3工具）" -ForegroundColor Yellow
    }
} else {
    Write-Host "❌ 数据库文件不存在" -ForegroundColor Red
    $hasIssues = $true
}

# 4. 检查编译状态
Write-Host "`n🔨 检查编译状态..." -ForegroundColor Yellow
$env:SQLX_OFFLINE = "false"
$compileResult = cargo check --message-format=short 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ 代码编译检查通过" -ForegroundColor Green
} else {
    Write-Host "❌ 编译检查失败:" -ForegroundColor Red
    Write-Host $compileResult -ForegroundColor Red
    $hasIssues = $true
}

# 5. 检查关键文件
Write-Host "`n📁 检查关键文件..." -ForegroundColor Yellow
$criticalFiles = @(
    "Cargo.toml",
    "src/main.rs",
    "migrations",
    "templates"
)

foreach ($file in $criticalFiles) {
    if (Test-Path $file) {
        Write-Host "✅ $file 存在" -ForegroundColor Green
    } else {
        Write-Host "❌ $file 缺失" -ForegroundColor Red
        $hasIssues = $true
    }
}

# 6. 自动修复建议
Write-Host "`n🔧 自动修复建议..." -ForegroundColor Cyan

if ($hasIssues) {
    Write-Host "⚠️  发现问题，建议执行以下修复步骤:" -ForegroundColor Yellow
    
    Write-Host "`n📋 修复步骤:"
    Write-Host "1. 运行数据库迁移: cargo run --bin migrate" -ForegroundColor White
    Write-Host "2. 清理重建: cargo clean && cargo build" -ForegroundColor White
    Write-Host "3. 启动服务器: cargo run" -ForegroundColor White
    
    # 询问是否自动执行修复
    $autoFix = Read-Host "`n是否自动执行修复? (y/N)"
    if ($autoFix -eq "y" -or $autoFix -eq "Y") {
        Write-Host "`n🚀 开始自动修复..." -ForegroundColor Cyan
        
        # 步骤1: 运行迁移
        Write-Host "`n📊 运行数据库迁移..."
        $migrateResult = cargo run --bin migrate 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ 数据库迁移完成" -ForegroundColor Green
        } else {
            Write-Host "❌ 数据库迁移失败: $migrateResult" -ForegroundColor Red
        }
        
        # 步骤2: 清理重建
        Write-Host "`n🧹 清理和重建项目..."
        cargo clean
        $buildResult = cargo build 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ 项目构建成功" -ForegroundColor Green
            
            # 步骤3: 启动服务器
            Write-Host "`n🚀 启动服务器..."
            Write-Host "服务器将在 http://localhost:3000 启动" -ForegroundColor Green
            Write-Host "按 Ctrl+C 停止服务器" -ForegroundColor Yellow
            cargo run
        } else {
            Write-Host "❌ 项目构建失败: $buildResult" -ForegroundColor Red
        }
    }
} else {
    Write-Host "✅ 所有检查通过，可以启动服务器!" -ForegroundColor Green
    
    $startServer = Read-Host "`n是否立即启动服务器? (Y/n)"
    if ($startServer -ne "n" -and $startServer -ne "N") {
        Write-Host "`n🚀 启动服务器..." -ForegroundColor Cyan
        Write-Host "服务器将在 http://localhost:3000 启动" -ForegroundColor Green
        Write-Host "按 Ctrl+C 停止服务器" -ForegroundColor Yellow
        cargo run
    }
}

Write-Host "`n" + "=" * 50
Write-Host "🎯 诊断完成！" -ForegroundColor Cyan

# 显示有用的链接
Write-Host "`n📱 快速访问链接:"
Write-Host "• 主页: http://localhost:3000" -ForegroundColor Blue
Write-Host "• 创建预警: http://localhost:3000/alert/new" -ForegroundColor Blue
Write-Host "• 市场API: http://localhost:3000/api/stocks/markets" -ForegroundColor Blue
Write-Host "• 测试页面: http://localhost:3000/../test_market_search.html" -ForegroundColor Blue

Write-Host "`n📚 问题库: docs/troubleshooting/README.md" -ForegroundColor Gray 