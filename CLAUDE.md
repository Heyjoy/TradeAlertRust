# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TradeAlert is a Rust-based trading alert system that monitors stock prices across multiple markets (A-shares, US stocks, crypto) and sends email notifications when price conditions are met. The system features a web interface, REST API, and automated price monitoring service.

## Development Commands

### Environment Setup
```bash
# Set required environment variables
$env:DATABASE_URL = "sqlite:data/trade_alert.db"
$env:SQLX_OFFLINE = "false"

# Load .env file (create from config/_env.example)
# Use double underscore format: TRADE_ALERT__EMAIL__SMTP_SERVER=smtp.gmail.com
```

### Development Workflow
```bash
# Start development server (runs migrations automatically)
.\scripts\dev_start.ps1

# Start demo mode for friend testing (data isolation)
.\scripts\start_demo.ps1      # Windows
./scripts/start_demo.sh       # Linux/macOS

# Create new database migration
.\scripts\new_migration.ps1 <migration_name>

# Run migrations manually
.\scripts\dev_migrate.ps1

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy

# Build release
cargo build --release
```

### Testing
```bash
# Test API endpoints
.\scripts\test\testing\test_api.ps1

# Test email functionality 
.\scripts\test\testing\test_email.ps1

# Test Yahoo Finance API
.\scripts\test\testing\test_yahoo_api.ps1
```

## Architecture

### Core Components
- **Web Server**: Axum-based HTTP server with REST API and HTML templates
- **Database**: SQLite with SQLx for async database operations
- **Price Service**: Yahoo Finance API integration with intelligent caching
- **Email Notifier**: SMTP email notifications with HTML templates
- **Alert Monitor**: Background service that checks price conditions every 30 seconds
- **Demo Mode**: User isolation system for safe friend testing with data separation

### Module Structure
```
src/
├── main.rs              # Entry point, HTTP routes, dependency injection
├── lib.rs               # Module exports
├── config/              # Configuration management with env var support
├── handlers/            # HTTP request handlers (market, strategy)
├── models/              # Data models and DTOs
├── services/            # Business logic
│   ├── db.rs           # Database operations and connection management
│   ├── email.rs        # SMTP email notification service
│   └── fetcher.rs      # Price fetching and alert monitoring
├── templates/           # Askama HTML templates
└── utils/               # Utility functions
```

### Key Design Patterns
- **Repository Pattern**: Database operations abstracted through `Database` service
- **Dependency Injection**: All services injected via `AppState` 
- **Error Handling**: Uses `anyhow` for application errors, `thiserror` for custom error types
- **Async/Await**: Full async architecture with Tokio runtime
- **Configuration**: Environment-based config with double underscore notation

### Price Data Strategy
The system uses a hybrid approach for price data:
1. Check database cache (valid for 1 hour)
2. If cache miss, fetch real-time data from Yahoo Finance API
3. Store fetched data in database for future requests
4. Background service updates all monitored symbols every 30 seconds

## Configuration

### Environment Variables
Use double underscore format for nested configuration:
```bash
TRADE_ALERT__EMAIL__SMTP_SERVER=smtp.gmail.com
TRADE_ALERT__EMAIL__SMTP_PORT=587
TRADE_ALERT__EMAIL__SMTP_USERNAME=your_email@gmail.com
TRADE_ALERT__EMAIL__SMTP_PASSWORD=your_app_password
TRADE_ALERT__DATABASE__URL=sqlite:data/trade_alert.db
TRADE_ALERT__LOGGING__LEVEL=info

# Demo mode configuration
TRADE_ALERT__DEMO__ENABLED=true
TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER=5
TRADE_ALERT__DEMO__DATA_RETENTION_HOURS=24
TRADE_ALERT__DEMO__DISABLE_EMAIL=true
```

### Database Migrations
- Located in `migrations/` directory
- Use `sqlx migrate` commands
- Always test migrations in development first
- Migration files follow format: `YYYYMMDDHHMMSS_description.sql`

## API Endpoints

### Alerts
- `GET /api/alerts` - List all alerts
- `POST /api/alerts` - Create new alert 
- `GET /api/alerts/{id}` - Get specific alert
- `PUT /api/alerts/{id}` - Update alert
- `DELETE /api/alerts/{id}` - Delete alert

### Price Data
- `GET /api/prices/{symbol}/latest` - Get latest price (real-time)
- `GET /api/prices/{symbol}` - Get price history

### Utilities
- `GET /api/test-email` - Send test email
- `GET /` - Dashboard web interface

## Important Implementation Details

### Email Configuration
- Support for multiple SMTP providers (Gmail, QQ, 163, Outlook)
- Gmail requires app-specific passwords
- HTML email templates using Askama
- Connection timeout issues may require trying different ports (587 vs 465)

### Alert Processing
- Background task runs every 30 seconds
- Checks all active alerts against current prices
- Updates alert status to 'triggered' when conditions met
- Sends email notifications automatically
- Prevents duplicate notifications for same alert

### Error Handling
- Never use `unwrap()` or `expect()` in production code
- All external API calls wrapped in proper error handling
- Database operations use transactions where appropriate
- Comprehensive logging with structured tracing

### Testing Strategy
- Unit tests for business logic
- Integration tests for API endpoints
- Mock external dependencies (Yahoo Finance API)
- Test email functionality separately

## Development Guidelines

### Code Style
- Follow Rust naming conventions (snake_case, PascalCase)
- Use `cargo fmt` for consistent formatting
- Add `#[derive(Debug)]` to all custom types
- Document public APIs with `///` comments

### Security
- Never log sensitive data (API keys, passwords)
- Use environment variables for all configuration
- Validate all input data
- Use HTTPS for external API calls

### Performance
- Use Arc<T> for shared state in async contexts
- Implement proper connection pooling for database
- Cache frequently accessed data appropriately
- Monitor external API rate limits

## 常见问题解决方案

### 编译问题

#### SQLx 相关编译错误
如果遇到 SQLx 编译错误，通常是以下原因：

1. **缺少查询缓存**：
   ```bash
   # 设置环境变量
   export DATABASE_URL="sqlite:data/trade_alert.db"
   export SQLX_OFFLINE=false
   
   # 更新查询缓存
   cargo sqlx prepare
   ```

2. **类型转换错误**（nullable 字段问题）：
   - 数据库字段为 nullable（通过 ALTER TABLE ADD COLUMN 添加）
   - 模型中定义为非 nullable 类型
   - 解决方法：在查询中使用 `COALESCE` 提供默认值，并使用 `"field!"` 类型注解

3. **Rust 环境配置**：
   ```bash
   # 检查 Rust 是否安装
   which cargo
   
   # 如果未找到，添加到 PATH
   export PATH="$HOME/.cargo/bin:$PATH"
   
   # 永久配置（添加到 ~/.bashrc 或 ~/.zshrc）
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   ```

#### 数据库迁移问题
```bash
# 检查迁移状态
sqlx migrate info

# 应用待处理的迁移
sqlx migrate run

# 如果需要重置数据库
rm data/trade_alert.db
sqlx migrate run
```

### 运行时问题

#### 邮件发送失败
1. 检查 SMTP 配置
2. Gmail 用户确保使用应用专用密码
3. 尝试不同的端口（587 vs 465）

#### API 连接超时
1. 检查网络连接
2. 验证 Yahoo Finance API 可访问性
3. 调整请求超时配置

### 开发环境检查清单
运行项目前的环境检查：
```bash
# 1. 检查 Rust 环境
cargo --version

# 2. 检查数据库文件
ls -la data/trade_alert.db

# 3. 检查环境变量
echo $DATABASE_URL

# 4. 检查依赖
cargo check

# 5. 运行测试
cargo test
```

### 环境检查脚本
可以创建以下脚本来自动检查开发环境：

```bash
#!/bin/bash
# scripts/check_dev_env.sh
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

# 检查数据库
echo "4. 检查数据库..."
if [ -f "data/trade_alert.db" ]; then
    echo "✅ 数据库文件存在"
else
    echo "⚠️  数据库文件不存在，将在首次运行时创建"
fi

# 检查环境变量
echo "5. 检查环境变量..."
if [ -n "$DATABASE_URL" ]; then
    echo "✅ DATABASE_URL 已设置: $DATABASE_URL"
else
    echo "⚠️  DATABASE_URL 未设置"
    echo "   设置命令: export DATABASE_URL=\"sqlite:data/trade_alert.db\""
fi

# 检查 SQLx 查询缓存
echo "6. 检查 SQLx 查询缓存..."
if [ -d ".sqlx" ]; then
    echo "✅ SQLx 查询缓存存在"
else
    echo "⚠️  SQLx 查询缓存不存在"
    echo "   生成命令: cargo sqlx prepare"
fi

# 检查编译状态
echo "7. 检查编译状态..."
if cargo check --quiet 2>/dev/null; then
    echo "✅ 项目编译检查通过"
else
    echo "❌ 项目编译检查失败，运行 'cargo check' 查看详细错误"
fi

echo "=========================="
echo "🎉 环境检查完成"
```

## 相关文档和资源

### 🔗 AI 协作相关
- **[AI 协作启动指南](docs/START_HERE.md)** - 每次对话必读的协作流程
- **[文档索引](docs/INDEX.md)** - 完整项目文档导航
- **[AI 协作经验](docs/dev/ai-collaboration-insights.md)** - 实战协作心得

### 🔧 故障排除资源
- **[故障排除总览](docs/troubleshooting/README.md)** - 常见问题分类和快速解决
- **[SQLx 编译问题详解](docs/troubleshooting/sqlx-compilation-issues.md)** - 数据库查询编译错误
- **[SQLx 调试心得](docs/dev/sqlx-debugging-experience.md)** - 实际问题解决经验沉淀
- **[构建问题](docs/troubleshooting/build-issues.md)** - 编译和构建问题
- **[启动问题](docs/troubleshooting/startup-issues.md)** - 运行时问题

### 🛠️ 开发工具
- **[环境检查脚本 (Linux)](scripts/check_dev_env.sh)** - 自动化环境诊断
- **[环境检查脚本 (Windows)](scripts/dev/development/check_dev_env.ps1)** - Windows 环境检查
- **[开发启动脚本](scripts/dev_start.ps1)** - 开发服务器启动
- **[演示模式脚本](scripts/start_demo.sh)** - 演示环境启动

### 📚 技术参考
- **[项目架构](docs/architecture/PROJECT_ARCHITECTURE.md)** - 完整系统架构
- **[开发计划](docs/dev/development/DEVELOPMENT_PLAN.md)** - 技术实现计划
- **[配置管理](docs/technical/CONFIGURATION_MANAGEMENT.md)** - 环境配置规范