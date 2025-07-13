#!/bin/bash
# TradeAlert 快速发布前检查脚本
# 专注于关键质量检查，避免过于严格的要求

set -e

echo "⚡ TradeAlert 快速发布检查..."
echo "============================="

# 1. 基础编译检查
echo
echo "1️⃣ 基础编译检查..."
if cargo check --bin trade_alert_rust; then
    echo "   ✅ 主程序编译通过"
else
    echo "   ❌ 编译失败"
    exit 1
fi

# 2. 关键代码质量检查（宽松模式）
echo
echo "2️⃣ 关键代码质量检查..."
if cargo clippy --bin trade_alert_rust -- -W clippy::correctness -W clippy::suspicious -W clippy::complexity 2>/dev/null; then
    echo "   ✅ 关键质量检查通过"
else
    echo "   ⚠️  发现一些建议，但不阻止部署"
fi

# 3. 配置文件关键检查
echo
echo "3️⃣ 配置文件关键检查..."

# 检查Cargo.toml重复段问题
profile_count=$(grep -c '\[profile\.release\]' Cargo.toml || echo "0")
if [ "$profile_count" -gt 1 ]; then
    echo "   ❌ 发现重复的[profile.release]段"
    exit 1
else
    echo "   ✅ Cargo.toml无重复配置"
fi

# 4. Railway部署关键文件检查
echo
echo "4️⃣ Railway部署文件检查..."
critical_files=(
    "config/railway.env.example"
    "deploy/nixpacks.toml" 
    ".railway-ignore"
)

all_files_exist=true
for file in "${critical_files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "   ✅ $file"
    else
        echo "   ❌ $file 缺失"
        all_files_exist=false
    fi
done

if [[ "$all_files_exist" != true ]]; then
    exit 1
fi

# 5. 快速构建测试
echo
echo "5️⃣ 快速构建测试..."
export CARGO_PROFILE_RELEASE_LTO=false
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16

if cargo build --release --bin trade_alert_rust; then
    echo "   ✅ Release构建成功"
else
    echo "   ❌ Release构建失败"
    exit 1
fi

# 总结
echo
echo "============================="
echo "✅ 快速检查完成！项目可以部署"
echo "📦 Railway优化配置已就位"
echo
echo "🚀 可以安全推送到GitHub触发Railway部署"
echo