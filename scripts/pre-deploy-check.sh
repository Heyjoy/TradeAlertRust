#!/bin/bash
# TradeAlert 发布前质量检查脚本 (Linux/macOS版本)
# 确保代码质量和配置正确性

set -e

echo "🚀 TradeAlert 发布前检查开始..."
echo "======================================"

# 1. 检查Rust环境
echo
echo "1️⃣ 检查Rust环境..."
if command -v cargo &> /dev/null; then
    echo "   ✅ Cargo: $(cargo --version)"
    echo "   ✅ Rustc: $(rustc --version)"
else
    echo "   ❌ Rust环境未找到"
    exit 1
fi

# 2. 代码格式检查
echo
echo "2️⃣ 代码格式检查..."
if cargo fmt --check; then
    echo "   ✅ 代码格式正确"
else
    echo "   ❌ 代码格式不符合标准，运行 'cargo fmt' 修复"
    exit 1
fi

# 3. Clippy静态分析（严格模式）
echo
echo "3️⃣ Clippy静态分析（严格模式）..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo "   ✅ Clippy检查通过"
else
    echo "   ❌ Clippy发现问题，请修复后重试"
    exit 1
fi

# 4. 编译检查
echo
echo "4️⃣ 编译检查..."
if cargo check --all-targets --all-features; then
    echo "   ✅ 编译检查通过"
else
    echo "   ❌ 编译检查失败"
    exit 1
fi

# 5. 测试运行
echo
echo "5️⃣ 运行测试套件..."
if cargo test --verbose; then
    echo "   ✅ 所有测试通过"
else
    echo "   ❌ 测试失败"
    exit 1
fi

# 6. 配置文件验证
echo
echo "6️⃣ 配置文件验证..."

# 检查Cargo.toml语法
if grep -q '\[profile\.release\].*\[profile\.release\]' Cargo.toml 2>/dev/null; then
    echo "   ❌ 发现重复的[profile.release]段"
    exit 1
else
    echo "   ✅ Cargo.toml格式正确"
fi

# 检查关键配置文件存在
config_files=(
    "config/config.toml.example"
    "config/railway.env.example" 
    "deploy/nixpacks.toml"
    "deploy/railway.toml"
    ".dockerignore"
    ".railway-ignore"
)

for file in "${config_files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "   ✅ $file 存在"
    else
        echo "   ❌ $file 缺失"
        exit 1
    fi
done

# 7. 依赖安全检查（如果有cargo-audit）
echo
echo "7️⃣ 依赖安全检查..."
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        echo "   ✅ 依赖安全检查通过"
    else
        echo "   ❌ 发现安全漏洞，请检查依赖"
        exit 1
    fi
else
    echo "   ⚠️  cargo-audit未安装，跳过安全检查"
    echo "   💡 安装命令: cargo install cargo-audit"
fi

# 8. Git状态检查
echo
echo "8️⃣ Git状态检查..."
if [[ -n $(git status --porcelain) ]]; then
    echo "   ⚠️  有未提交的更改:"
    git status --porcelain
    echo "   💡 建议先提交所有更改"
else
    echo "   ✅ Git工作区干净"
fi

# 9. 构建优化验证
echo
echo "9️⃣ 构建优化验证..."
if grep -q 'lto.*=.*false' Cargo.toml && grep -q 'codegen-units.*=.*16' Cargo.toml; then
    echo "   ✅ Railway构建优化已配置"
else
    echo "   ⚠️  构建优化配置可能缺失"
fi

# 总结
echo
echo "======================================"
echo "✅ 发布前检查完成！"
echo "📦 项目已准备好部署到Railway"
echo
echo "💡 下一步:"
echo "   1. git add . && git commit -m 'your message'"
echo "   2. git push origin master"
echo "   3. 在Railway Dashboard监控部署状态"
echo