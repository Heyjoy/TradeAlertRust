#!/bin/bash
# TradeAlert 开发环境检查脚本
# 使用方法: ./scripts/check_dev_env.sh

echo "🔍 TradeAlert 开发环境检查"
echo "=========================="

# 检查 Rust 环境
echo "1. 检查 Rust 环境..."
if command -v cargo &> /dev/null; then
    echo "✅ Cargo 版本: $(cargo --version)"
else
    echo "❌ Cargo 未找到，请安装 Rust"
    echo "   安装命令: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# 检查 PATH 配置
echo "2. 检查 PATH 配置..."
if [[ ":$PATH:" == *":$HOME/.cargo/bin:"* ]]; then
    echo "✅ Rust 已添加到 PATH"
else
    echo "⚠️  Rust 未添加到 PATH，可能需要手动设置"
    echo "   添加命令: export PATH=\"\$HOME/.cargo/bin:\$PATH\""
fi

# 检查 SQLx CLI
echo "3. 检查 SQLx CLI..."
if command -v sqlx &> /dev/null; then
    echo "✅ SQLx CLI 可用"
else
    echo "⚠️  SQLx CLI 未安装"
    echo "   安装命令: cargo install sqlx-cli --features sqlite"
fi

# 检查数据库目录
echo "4. 检查数据库目录..."
if [ -d "data" ]; then
    echo "✅ 数据目录存在"
    if [ -f "data/trade_alert.db" ]; then
        echo "✅ 数据库文件存在"
    else
        echo "⚠️  数据库文件不存在，将在首次运行时创建"
    fi
else
    echo "⚠️  数据目录不存在，将创建"
    mkdir -p data
fi

# 检查环境变量
echo "5. 检查环境变量..."
if [ -n "$DATABASE_URL" ]; then
    echo "✅ DATABASE_URL 已设置: $DATABASE_URL"
else
    echo "⚠️  DATABASE_URL 未设置"
    echo "   设置命令: export DATABASE_URL=\"sqlite:data/trade_alert.db\""
fi

# 检查 .env 文件
echo "6. 检查配置文件..."
if [ -f ".env" ]; then
    echo "✅ .env 文件存在"
elif [ -f "config/.env" ]; then
    echo "✅ config/.env 文件存在"
else
    echo "⚠️  环境配置文件不存在"
    echo "   创建命令: cp config/_env.example .env"
fi

# 检查 SQLx 查询缓存
echo "7. 检查 SQLx 查询缓存..."
if [ -d ".sqlx" ]; then
    echo "✅ SQLx 查询缓存存在"
    cache_files=$(ls .sqlx/query-*.json 2>/dev/null | wc -l)
    echo "   缓存文件数量: $cache_files"
else
    echo "⚠️  SQLx 查询缓存不存在"
    echo "   生成命令: cargo sqlx prepare"
fi

# 检查编译状态
echo "8. 检查编译状态..."
echo "   正在检查项目编译状态..."
if cargo check --quiet 2>/dev/null; then
    echo "✅ 项目编译检查通过"
else
    echo "❌ 项目编译检查失败"
    echo "   运行 'cargo check' 查看详细错误信息"
fi

# 检查测试状态
echo "9. 检查测试状态..."
echo "   正在运行测试..."
if cargo test --quiet 2>/dev/null; then
    echo "✅ 所有测试通过"
else
    echo "⚠️  部分测试失败或跳过"
    echo "   运行 'cargo test' 查看详细测试结果"
fi

# 总结
echo "=========================="
echo "🎉 环境检查完成"
echo ""
echo "💡 如果发现问题，请参考:"
echo "   - 完整文档: docs/troubleshooting/"
echo "   - SQLx 问题: docs/troubleshooting/sqlx-compilation-issues.md"
echo "   - 启动问题: docs/troubleshooting/startup-issues.md"
echo ""
echo "🚀 环境正常时，可以运行:"
echo "   ./scripts/dev_start.ps1    # 启动开发服务器"
echo "   ./scripts/start_demo.sh    # 启动演示模式"