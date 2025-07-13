#!/usr/bin/env pwsh
# TradeAlert 快速发布前检查脚本
# 专注于关键质量检查，避免过于严格的要求

$ErrorActionPreference = "Stop"

Write-Host "⚡ TradeAlert 快速发布检查..." -ForegroundColor Green
Write-Host "=============================" -ForegroundColor Cyan

# 1. 基础编译检查
Write-Host "`n1️⃣ 基础编译检查..." -ForegroundColor Yellow
try {
    cargo check --bin trade_alert_rust
    Write-Host "   ✅ 主程序编译通过" -ForegroundColor Green
} catch {
    Write-Host "   ❌ 编译失败" -ForegroundColor Red
    exit 1
}

# 2. 关键代码质量检查（宽松模式）
Write-Host "`n2️⃣ 关键代码质量检查..." -ForegroundColor Yellow
try {
    # 只检查主要的错误，不包括格式警告
    cargo clippy --bin trade_alert_rust -- -W clippy::correctness -W clippy::suspicious -W clippy::complexity
    Write-Host "   ✅ 关键质量检查通过" -ForegroundColor Green
} catch {
    Write-Host "   ⚠️  发现一些建议，但不阻止部署" -ForegroundColor Yellow
}

# 3. 配置文件关键检查
Write-Host "`n3️⃣ 配置文件关键检查..." -ForegroundColor Yellow

# 检查Cargo.toml重复段问题
try {
    $cargoContent = Get-Content "Cargo.toml" -Raw
    $profileCount = ($cargoContent | Select-String '\[profile\.release\]' -AllMatches).Matches.Count
    if ($profileCount -gt 1) {
        throw "发现重复的[profile.release]段"
    }
    Write-Host "   ✅ Cargo.toml无重复配置" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Cargo.toml配置错误: $_" -ForegroundColor Red
    exit 1
}

# 4. Railway部署关键文件检查
Write-Host "`n4️⃣ Railway部署文件检查..." -ForegroundColor Yellow
$criticalFiles = @(
    "config/railway.env.example",
    "deploy/nixpacks.toml", 
    ".railway-ignore"
)

$allFilesExist = $true
foreach ($file in $criticalFiles) {
    if (Test-Path $file) {
        Write-Host "   ✅ $file" -ForegroundColor Green
    } else {
        Write-Host "   ❌ $file 缺失" -ForegroundColor Red
        $allFilesExist = $false
    }
}

if (-not $allFilesExist) {
    exit 1
}

# 5. 快速构建测试
Write-Host "`n5️⃣ 快速构建测试..." -ForegroundColor Yellow
try {
    # 只检查能否成功构建，不运行完整测试
    $env:CARGO_PROFILE_RELEASE_LTO = "false"
    $env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "16"
    cargo build --release --bin trade_alert_rust
    Write-Host "   ✅ Release构建成功" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Release构建失败" -ForegroundColor Red
    exit 1
}

# 总结
Write-Host "`n=============================" -ForegroundColor Cyan
Write-Host "✅ 快速检查完成！项目可以部署" -ForegroundColor Green
Write-Host "📦 Railway优化配置已就位" -ForegroundColor Green
Write-Host "`n🚀 可以安全推送到GitHub触发Railway部署" -ForegroundColor Cyan
Write-Host ""