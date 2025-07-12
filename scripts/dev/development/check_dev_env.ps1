# Development Environment Check Script
# 检查Cursor AI开发环境配置

param(
    [switch]$Verbose
)

Write-Host "TradeAlertRust 开发环境检查" -ForegroundColor Cyan
Write-Host "=========================" -ForegroundColor Cyan

$ErrorCount = 0
$WarningCount = 0

function Write-Status($Message, $Status, $Details = "") {
    $Icon = if ($Status -eq "OK") { "[OK]" } elseif ($Status -eq "WARN") { "[WARN]" } else { "[ERROR]" }
    $Color = if ($Status -eq "OK") { "Green" } elseif ($Status -eq "WARN") { "Yellow" } else { "Red" }
    
    Write-Host "$Icon $Message" -ForegroundColor $Color
    
    if ($Details -and $Verbose) {
        Write-Host "   $Details" -ForegroundColor Gray
    }
    
    if ($Status -eq "ERROR") { $script:ErrorCount++ }
    if ($Status -eq "WARN") { $script:WarningCount++ }
}

# 检查基础工具
Write-Host "`n基础工具检查:" -ForegroundColor Yellow

# Rust
$RustVersion = & rustc --version 2>$null
if ($RustVersion) {
    Write-Status "Rust 编译器" "OK" $RustVersion
} else {
    Write-Status "Rust 编译器" "ERROR" "未安装"
}

# Cargo
$CargoVersion = & cargo --version 2>$null
if ($CargoVersion) {
    Write-Status "Cargo 包管理器" "OK" $CargoVersion
} else {
    Write-Status "Cargo 包管理器" "ERROR" "未安装"
}

# Git
$GitVersion = & git --version 2>$null
if ($GitVersion) {
    Write-Status "Git 版本控制" "OK" $GitVersion
} else {
    Write-Status "Git 版本控制" "ERROR" "未安装"
}

# 检查项目文件
Write-Host "`n项目文件检查:" -ForegroundColor Yellow

$RequiredFiles = @(
    ".cursorrules",
    ".cursor/rules/rust-rules.mdc",
    ".cursor/rules/trading-rules.mdc", 
    ".cursor/rules/security-rules.mdc",
    ".cursor/modes.json",
    "Cargo.toml",
    "src/main.rs",
    "docs/AI_CONTEXT.md",
    "tasks/current-tasks.md",
    "docs/development-status.md"
)

foreach ($File in $RequiredFiles) {
    if (Test-Path $File) {
        Write-Status "文件: $File" "OK"
    } else {
        Write-Status "文件: $File" "ERROR" "缺失"
    }
}

# 检查Cursor配置
Write-Host "`nCursor AI 配置检查:" -ForegroundColor Yellow

if (Test-Path ".cursorrules") {
    $Content = Get-Content ".cursorrules" -Raw -ErrorAction SilentlyContinue
    if ($Content -and $Content.Contains("TradeAlertRust")) {
        Write-Status ".cursorrules 配置" "OK" "包含项目规则"
    } else {
        Write-Status ".cursorrules 配置" "WARN" "内容可能不完整"
    }
} else {
    Write-Status ".cursorrules 配置" "ERROR" "文件不存在"
}

if (Test-Path ".cursor/modes.json") {
    try {
        $ModesConfig = Get-Content ".cursor/modes.json" | ConvertFrom-Json -ErrorAction Stop
        $ModeCount = $ModesConfig.modes.PSObject.Properties.Count
        Write-Status "AI Agent 模式" "OK" "配置了 $ModeCount 个模式"
    } catch {
        Write-Status "AI Agent 模式" "ERROR" "JSON格式错误"
    }
} else {
    Write-Status "AI Agent 模式" "ERROR" "modes.json不存在"
}

# 检查Rust项目
Write-Host "`nRust 项目检查:" -ForegroundColor Yellow

if (Test-Path "Cargo.toml") {
    $CargoContent = Get-Content "Cargo.toml" -Raw -ErrorAction SilentlyContinue
    $RequiredDeps = @("tokio", "axum", "sqlx", "serde", "tracing")
    
    foreach ($Dep in $RequiredDeps) {
        if ($CargoContent -and $CargoContent.Contains($Dep)) {
            Write-Status "依赖: $Dep" "OK"
        } else {
            Write-Status "依赖: $Dep" "WARN" "可能缺失"
        }
    }
} else {
    Write-Status "Cargo.toml" "ERROR" "文件不存在"
}

# 检查数据库和SQLx配置
Write-Host "`n数据库和SQLx检查:" -ForegroundColor Yellow

# 检查数据库目录
if (Test-Path "data") {
    Write-Status "数据目录" "OK"
    if (Test-Path "data/trade_alert.db") {
        Write-Status "数据库文件" "OK"
    } else {
        Write-Status "数据库文件" "WARN" "将在首次运行时创建"
    }
} else {
    Write-Status "数据目录" "WARN" "不存在，将创建"
    New-Item -ItemType Directory -Path "data" -Force | Out-Null
}

# 检查SQLx查询缓存
if (Test-Path ".sqlx") {
    $CacheFiles = Get-ChildItem ".sqlx" -Filter "query-*.json" -ErrorAction SilentlyContinue
    $Count = if ($CacheFiles) { $CacheFiles.Count } else { 0 }
    Write-Status "SQLx查询缓存" "OK" "包含 $Count 个缓存文件"
} else {
    Write-Status "SQLx查询缓存" "WARN" "不存在，需要运行 cargo sqlx prepare"
}

# 检查环境变量
$DatabaseUrl = $env:DATABASE_URL
if ($DatabaseUrl) {
    Write-Status "DATABASE_URL" "OK" $DatabaseUrl
} else {
    Write-Status "DATABASE_URL" "WARN" "未设置"
}

# 检查SQLx CLI
$SqlxVersion = & sqlx --version 2>$null
if ($SqlxVersion) {
    Write-Status "SQLx CLI" "OK" $SqlxVersion
} else {
    Write-Status "SQLx CLI" "WARN" "未安装，运行: cargo install sqlx-cli --features sqlite"
}

# 编译检查
Write-Host "   检查编译状态..." -ForegroundColor Gray
$CheckResult = & cargo check --quiet 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Status "项目编译" "OK" "无错误"
} else {
    Write-Status "项目编译" "ERROR" "存在错误，可能是SQLx相关问题"
    if ($Verbose) {
        Write-Host "   编译错误: $CheckResult" -ForegroundColor Red
        Write-Host "   参考文档: docs/troubleshooting/sqlx-compilation-issues.md" -ForegroundColor Cyan
    }
}

# 总结
Write-Host "`n检查总结:" -ForegroundColor Cyan
Write-Host "========" -ForegroundColor Cyan

if ($ErrorCount -eq 0 -and $WarningCount -eq 0) {
    Write-Host "恭喜！开发环境配置完美！" -ForegroundColor Green
} elseif ($ErrorCount -eq 0) {
    Write-Host "开发环境基本就绪，有 $WarningCount 个警告" -ForegroundColor Yellow
} else {
    Write-Host "发现 $ErrorCount 个错误和 $WarningCount 个警告" -ForegroundColor Red
}

Write-Host "错误数量: $ErrorCount" -ForegroundColor $(if ($ErrorCount -eq 0) { "Green" } else { "Red" })
Write-Host "警告数量: $WarningCount" -ForegroundColor $(if ($WarningCount -eq 0) { "Green" } else { "Yellow" })

if ($ErrorCount -gt 0 -or $WarningCount -gt 0) {
    Write-Host "`n建议:" -ForegroundColor Cyan
    if ($ErrorCount -gt 0) {
        Write-Host "1. 安装缺失的工具和依赖" -ForegroundColor Yellow
        Write-Host "2. 创建缺失的配置文件" -ForegroundColor Yellow
        Write-Host "3. 修复编译错误" -ForegroundColor Yellow
    }
    if ($WarningCount -gt 0) {
        Write-Host "4. 完善配置文件内容" -ForegroundColor Yellow
    }
}

Write-Host "`n准备开始 AI 协作开发！" -ForegroundColor Green
exit $ErrorCount 