#!/bin/bash
# Railway部署本地模拟脚本 (Linux/macOS版本)
# 模拟Railway环境来排查部署问题

echo "🚀 Railway部署本地模拟测试"
echo "================================"

# 1. 设置Railway模拟环境变量
echo ""
echo "📝 设置Railway环境变量..."
export SQLX_OFFLINE=true
export TRADE_ALERT_EMAIL_SMTP_SERVER=smtp.gmail.com
export TRADE_ALERT_EMAIL_SMTP_PORT=587
export TRADE_ALERT_EMAIL_SMTP_USERNAME=your-email@gmail.com
export TRADE_ALERT_EMAIL_SMTP_PASSWORD=your-app-password
export TRADE_ALERT_EMAIL_FROM_EMAIL=your-email@gmail.com
export TRADE_ALERT_EMAIL_TO_EMAIL=your-email@gmail.com
export TRADE_ALERT_EMAIL_ENABLED=true
export TRADE_ALERT_LOGGING_LEVEL=info

echo "✅ 环境变量已设置 (使用SQLX_OFFLINE=true)"

# 2. 生成SQLx离线缓存（如果需要）
echo ""
echo "🗄️ 生成SQLx离线缓存..."
if [ -d ".sqlx" ]; then
    echo "⚠️  .sqlx目录已存在，跳过生成"
else
    echo "正在生成SQLx缓存文件..."
    export DATABASE_URL="sqlite:data/trade_alert.db"
    export SQLX_OFFLINE=false
    
    # 确保数据库是最新的
    cargo sqlx migrate run
    if [ $? -ne 0 ]; then
        echo "❌ 数据库迁移失败"
        exit 1
    fi
    
    # 生成离线缓存
    cargo sqlx prepare
    if [ $? -ne 0 ]; then
        echo "❌ SQLx缓存生成失败"
        exit 1
    fi
    
    echo "✅ SQLx缓存生成完成"
    export SQLX_OFFLINE=true
fi

# 3. 模拟Railway构建过程
echo ""
echo "🔨 模拟Railway构建过程..."
echo "执行: cargo build --release"

cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ 构建失败！这就是Railway部署失败的原因"
    echo ""
    echo "🔍 可能的解决方案:"
    echo "1. 检查SQLx离线模式是否正确设置"
    echo "2. 确保所有数据库迁移都已运行"
    echo "3. 检查代码语法错误"
    exit 1
fi

echo "✅ 构建成功！"

# 4. 测试启动
echo ""
echo "🚀 测试应用启动..."
echo "执行: cargo run --release"
echo "⚠️  按Ctrl+C停止测试"

# 使用timeout命令限制运行时间
timeout 15s cargo run --release &
PID=$!

# 等待启动
sleep 2

# 检查进程是否还在运行
if kill -0 $PID 2>/dev/null; then
    echo "✅ 应用启动成功！Railway部署应该可以正常工作"
    # 停止进程
    kill $PID 2>/dev/null
    wait $PID 2>/dev/null
else
    echo "⚠️  应用启动失败"
fi

echo ""
echo "📋 测试完成!"
echo "================================"

# 5. 输出Railway部署建议
echo ""
echo "🎯 Railway部署建议:"
echo "1. 确保在Railway中设置 SQLX_OFFLINE=true"
echo "2. 提交 .sqlx/ 目录到Git仓库"
echo "3. 检查railway.toml中的startCommand是否正确"
echo "4. 确保所有必需的环境变量都在Railway中配置"

echo ""
echo "📚 详细故障排除指南: docs/troubleshooting/railway-deployment-issues.md"