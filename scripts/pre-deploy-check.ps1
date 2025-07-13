#!/usr/bin/env pwsh
# TradeAlert 发布前质量检查脚本
# 确保代码质量和配置正确性

$ErrorActionPreference = "Stop"

Write-Host "🚀 TradeAlert 发布前检查开始..." -ForegroundColor Green
Write-Host "======================================" -ForegroundColor Cyan

# 1. 检查Rust环境
Write-Host "`n1️⃣ 检查Rust环境..." -ForegroundColor Yellow
try {
    $rustVersion = cargo --version
    $rustcVersion = rustc --version
    Write-Host "   ✅ Cargo: $rustVersion" -ForegroundColor Green
    Write-Host "   ✅ Rustc: $rustcVersion" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Rust环境检查失败: $_" -ForegroundColor Red
    exit 1
}

# 2. 代码格式检查
Write-Host "`n2️⃣ 代码格式检查..." -ForegroundColor Yellow
try {
    cargo fmt --check
    Write-Host "   ✅ 代码格式正确" -ForegroundColor Green
} catch {
    Write-Host "   ❌ 代码格式不符合标准，运行 'cargo fmt' 修复" -ForegroundColor Red
    exit 1
}

# 3. Clippy静态分析（严格模式）
Write-Host "`n3️⃣ Clippy静态分析（严格模式）..." -ForegroundColor Yellow
try {
    cargo clippy --all-targets --all-features -- -D warnings
    Write-Host "   ✅ Clippy检查通过" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Clippy发现问题，请修复后重试" -ForegroundColor Red
    exit 1
}

# 4. 编译检查
Write-Host "`n4️⃣ 编译检查..." -ForegroundColor Yellow
try {
    cargo check --all-targets --all-features
    Write-Host "   ✅ 编译检查通过" -ForegroundColor Green
} catch {
    Write-Host "   ❌ 编译检查失败" -ForegroundColor Red
    exit 1
}

# 5. 测试运行
Write-Host "`n5️⃣ 运行测试套件..." -ForegroundColor Yellow
try {
    cargo test --verbose
    Write-Host "   ✅ 所有测试通过" -ForegroundColor Green
} catch {
    Write-Host "   ❌ 测试失败" -ForegroundColor Red
    exit 1
}

# 6. 配置文件验证
Write-Host "`n6️⃣ 配置文件验证..." -ForegroundColor Yellow

# 检查Cargo.toml语法
try {
    $cargoContent = Get-Content "Cargo.toml" -Raw
    if ($cargoContent -match '\[profile\.release\].*\[profile\.release\]') {
        throw "发现重复的[profile.release]段"
    }
    Write-Host "   ✅ Cargo.toml格式正确" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Cargo.toml验证失败: $_" -ForegroundColor Red
    exit 1
}

# 检查关键配置文件存在
$configFiles = @(
    "config/config.toml.example",
    "config/railway.env.example", 
    "deploy/nixpacks.toml",
    "deploy/railway.toml",
    ".dockerignore",
    ".railway-ignore"
)

foreach ($file in $configFiles) {
    if (Test-Path $file) {
        Write-Host "   ✅ $file 存在" -ForegroundColor Green
    } else {
        Write-Host "   ❌ $file 缺失" -ForegroundColor Red
        exit 1
    }
}

# 7. 依赖安全检查（如果有cargo-audit）
Write-Host "`n7️⃣ 依赖安全检查..." -ForegroundColor Yellow
try {
    if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
        cargo audit
        Write-Host "   ✅ 依赖安全检查通过" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  cargo-audit未安装，跳过安全检查" -ForegroundColor Yellow
        Write-Host "   💡 安装命令: cargo install cargo-audit" -ForegroundColor Cyan
    }
} catch {
    Write-Host "   ❌ 发现安全漏洞，请检查依赖" -ForegroundColor Red
    exit 1
}

# 8. Git状态检查
Write-Host "`n8️⃣ Git状态检查..." -ForegroundColor Yellow
try {
    $gitStatus = git status --porcelain
    if ($gitStatus) {
        Write-Host "   ⚠️  有未提交的更改:" -ForegroundColor Yellow
        Write-Host $gitStatus -ForegroundColor Cyan
        Write-Host "   💡 建议先提交所有更改" -ForegroundColor Cyan
    } else {
        Write-Host "   ✅ Git工作区干净" -ForegroundColor Green
    }
} catch {
    Write-Host "   ❌ Git检查失败: $_" -ForegroundColor Red
}

# 9. 构建优化验证
Write-Host "`n9️⃣ 构建优化验证..." -ForegroundColor Yellow
try {
    # 检查release profile设置
    $cargoContent = Get-Content "Cargo.toml" -Raw
    if ($cargoContent -match 'lto\s*=\s*false' -and $cargoContent -match 'codegen-units\s*=\s*16') {
        Write-Host "   ✅ Railway构建优化已配置" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  构建优化配置可能缺失" -ForegroundColor Yellow
    }
} catch {
    Write-Host "   ❌ 构建配置检查失败: $_" -ForegroundColor Red
}

# 总结
Write-Host "`n======================================" -ForegroundColor Cyan
Write-Host "✅ 发布前检查完成！" -ForegroundColor Green
Write-Host "📦 项目已准备好部署到Railway" -ForegroundColor Green
Write-Host "`n💡 下一步:" -ForegroundColor Cyan
Write-Host "   1. git add . && git commit -m 'your message'" -ForegroundColor White
Write-Host "   2. git push origin master" -ForegroundColor White
Write-Host "   3. 在Railway Dashboard监控部署状态" -ForegroundColor White
Write-Host ""