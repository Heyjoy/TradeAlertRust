#!/bin/bash
# 演示模式启动脚本 (Linux/macOS)
# 使用方法: ./scripts/start_demo.sh

set -e

echo "🔬 启动演示模式..."

# 设置演示模式环境变量
export DATABASE_URL="sqlite:data/demo_trade_alert.db"
export SQLX_OFFLINE="false"

# 演示模式配置
export TRADE_ALERT__DEMO__ENABLED="true"
export TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER="5"
export TRADE_ALERT__DEMO__DATA_RETENTION_HOURS="24"
export TRADE_ALERT__DEMO__DISABLE_EMAIL="true"
export TRADE_ALERT__DEMO__SHOW_DEMO_BANNER="true"
export TRADE_ALERT__DEMO__RATE_LIMIT_PER_MINUTE="20"

# 禁用邮件发送
export TRADE_ALERT__EMAIL__ENABLED="false"

echo "📋 演示模式配置:"
echo "  - 数据库: $DATABASE_URL"
echo "  - 每用户最大预警数: $TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER"
echo "  - 数据保留时间: $TRADE_ALERT__DEMO__DATA_RETENTION_HOURS 小时"
echo "  - 邮件通知: 已禁用"
echo "  - 用户隔离: 已启用"

# 创建演示数据库目录
if [ ! -d "data" ]; then
    mkdir -p data
    echo "✅ 创建数据目录"
fi

# 检查并创建演示数据库
if [ -f "data/trade_alert.db" ] && [ ! -f "data/demo_trade_alert.db" ]; then
    echo "🔄 创建演示数据库..."
    cp "data/trade_alert.db" "data/demo_trade_alert.db"
    echo "✅ 演示数据库已创建"
elif [ ! -f "data/demo_trade_alert.db" ]; then
    echo "📋 将创建新的演示数据库"
else
    echo "📋 使用现有演示数据库"
fi

# 运行数据库迁移
echo "🔄 运行数据库迁移..."
if ! sqlx migrate run; then
    echo "❌ 迁移失败"
    echo "💡 请确保已安装 sqlx-cli: cargo install sqlx-cli"
    exit 1
fi
echo "✅ 数据库迁移完成!"

# 显示启动信息
echo ""
echo "🚀 启动演示环境..."
echo "📍 应用将在 http://127.0.0.1:3000 启动"
echo "🔬 演示模式特性:"
echo "   • 用户数据隔离 - 每个访问者看到独立的数据"
echo "   • 预警数量限制 - 每用户最多${TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER}个预警"
echo "   • 数据自动清理 - 24小时后自动删除演示数据"
echo "   • 邮件通知禁用 - 不会发送真实邮件"
echo "   • 演示横幅显示 - 提醒用户当前为演示环境"
echo ""
echo "⏹️  按 Ctrl+C 停止演示环境"
echo "🌐 分享给朋友测试: http://localhost:3000?demo=true"
echo ""

# 启动应用
cargo run --bin trade_alert_rust