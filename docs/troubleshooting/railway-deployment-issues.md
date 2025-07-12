# Railway部署故障排除指南

## 🚨 常见部署失败原因

### 1. **SQLx编译时数据库检查失败**
**错误信息**：
```
error: error returned from database: (code: 1) no such table: strategy_signals
```

**原因**：Railway构建环境没有运行数据库迁移
**解决方案**：使用离线模式

#### 方法1：强制离线模式（推荐）
在Railway环境变量中添加：
```
SQLX_OFFLINE=true
```

#### 方法2：生成离线缓存文件
本地执行：
```bash
# 确保本地数据库最新
cargo sqlx migrate run

# 生成查询缓存文件
cargo sqlx prepare
```
将生成的`.sqlx/`目录提交到Git

### 2. **环境变量配置错误**
**错误信息**：
```
Error: Failed to load configuration
```

**必需的环境变量**：
```
TRADE_ALERT_EMAIL_SMTP_SERVER=smtp.gmail.com
TRADE_ALERT_EMAIL_SMTP_PORT=587
TRADE_ALERT_EMAIL_SMTP_USERNAME=你的Gmail地址
TRADE_ALERT_EMAIL_SMTP_PASSWORD=你的Gmail应用专用密码
TRADE_ALERT_EMAIL_FROM_EMAIL=你的Gmail地址
TRADE_ALERT_EMAIL_TO_EMAIL=你的Gmail地址
TRADE_ALERT_EMAIL_ENABLED=true
```

### 3. **构建超时**
**错误信息**：
```
Build timeout exceeded
```

**原因**：Rust编译时间过长
**解决方案**：
1. 使用Rust缓存
2. 优化依赖项
3. 使用离线模式避免数据库连接

### 4. **内存不足**
**错误信息**：
```
process killed (signal: SIGKILL)
```

**原因**：Railway免费层内存限制
**解决方案**：
1. 优化编译设置
2. 升级Railway计划
3. 使用release模式构建

### 5. **启动命令错误**
**错误信息**：
```
Error: no such file or directory: bin/trade_alert_rust
```

**检查**：
- `railway.toml`中的startCommand
- `nixpacks.toml`中的cmd
- Cargo.toml中的binary name

## 🛠️ 本地Railway环境模拟

### 方法1：使用Docker模拟Railway环境

创建Railway模拟环境：
```bash
# 创建模拟环境目录
mkdir railway-sim && cd railway-sim

# 复制项目文件
cp -r /path/to/TradeAlertRust/* .

# 创建Railway模拟环境变量文件
cat > .env << 'EOF'
SQLX_OFFLINE=true
TRADE_ALERT_EMAIL_SMTP_SERVER=smtp.gmail.com
TRADE_ALERT_EMAIL_SMTP_PORT=587
TRADE_ALERT_EMAIL_SMTP_USERNAME=your-email@gmail.com
TRADE_ALERT_EMAIL_SMTP_PASSWORD=your-app-password
TRADE_ALERT_EMAIL_FROM_EMAIL=your-email@gmail.com
TRADE_ALERT_EMAIL_TO_EMAIL=your-email@gmail.com
TRADE_ALERT_EMAIL_ENABLED=true
TRADE_ALERT_LOGGING_LEVEL=info
EOF

# 使用Railway构建器模拟
docker run --rm -v $(pwd):/workspace -w /workspace \
  --env-file .env \
  nixpacks/nixpacks:latest \
  build . --name trade-alert
```

### 方法2：本地环境变量模拟

#### Windows PowerShell
```powershell
# 设置Railway模拟环境变量
$env:SQLX_OFFLINE = "true"
$env:TRADE_ALERT_EMAIL_SMTP_SERVER = "smtp.gmail.com"
$env:TRADE_ALERT_EMAIL_SMTP_PORT = "587"
$env:TRADE_ALERT_EMAIL_SMTP_USERNAME = "your-email@gmail.com"
$env:TRADE_ALERT_EMAIL_SMTP_PASSWORD = "your-app-password"
$env:TRADE_ALERT_EMAIL_FROM_EMAIL = "your-email@gmail.com"
$env:TRADE_ALERT_EMAIL_TO_EMAIL = "your-email@gmail.com"
$env:TRADE_ALERT_EMAIL_ENABLED = "true"
$env:TRADE_ALERT_LOGGING_LEVEL = "info"

# 模拟Railway构建流程
cargo build --release
cargo run --release
```

#### Linux/macOS
```bash
# 创建Railway环境变量文件
cat > railway.env << 'EOF'
export SQLX_OFFLINE=true
export TRADE_ALERT_EMAIL_SMTP_SERVER=smtp.gmail.com
export TRADE_ALERT_EMAIL_SMTP_PORT=587
export TRADE_ALERT_EMAIL_SMTP_USERNAME=your-email@gmail.com
export TRADE_ALERT_EMAIL_SMTP_PASSWORD=your-app-password
export TRADE_ALERT_EMAIL_FROM_EMAIL=your-email@gmail.com
export TRADE_ALERT_EMAIL_TO_EMAIL=your-email@gmail.com
export TRADE_ALERT_EMAIL_ENABLED=true
export TRADE_ALERT_LOGGING_LEVEL=info
EOF

# 加载环境变量并测试
source railway.env
cargo build --release
cargo run --release
```

## 🔍 调试Railway部署

### 1. 查看构建日志
在Railway Dashboard:
1. 点击你的服务
2. 进入"Deployments"标签
3. 点击失败的部署
4. 查看详细日志

### 2. 本地复现构建过程
```bash
# 生成SQLx离线文件
cargo sqlx prepare

# 测试离线构建
SQLX_OFFLINE=true cargo build --release

# 测试启动
SQLX_OFFLINE=true cargo run --release
```

### 3. 检查Railway配置文件

#### railway.toml
```toml
[build]
builder = "NIXPACKS"

[deploy]
startCommand = "bin/trade_alert_rust"  # 确保二进制名称正确
numReplicas = 1
sleepApplication = false
restartPolicyType = "ON_FAILURE"
restartPolicyMaxRetries = 10
```

#### nixpacks.toml
```toml
[start]
cmd = "bin/trade_alert_rust"  # 确保与railway.toml一致
```

## 📋 部署前检查清单

### ✅ 代码准备
- [ ] 所有代码已提交并推送到GitHub
- [ ] 生成了SQLx离线缓存文件（如果使用）
- [ ] 构建配置文件存在且正确

### ✅ 环境变量
- [ ] 所有必需的环境变量已在Railway中配置
- [ ] Gmail应用专用密码已生成
- [ ] SQLX_OFFLINE设置为true（推荐）

### ✅ 本地测试
- [ ] 本地可以正常构建：`cargo build --release`
- [ ] 本地可以正常运行：`cargo run --release`
- [ ] 使用Railway环境变量本地测试通过

## 🚀 快速修复步骤

### 对于SQLx相关错误：
1. 在Railway中添加环境变量：`SQLX_OFFLINE=true`
2. 本地执行：`cargo sqlx prepare`
3. 提交`.sqlx/`目录到Git
4. 重新部署

### 对于配置错误：
1. 检查Railway环境变量是否完整
2. 确认Gmail应用专用密码正确
3. 本地使用相同环境变量测试

### 对于构建超时：
1. 添加`SQLX_OFFLINE=true`减少构建时间
2. 清理不必要的依赖
3. 考虑升级Railway计划

## 🔗 相关文档
- [Railway部署指南](../user/deployment/RAILWAY_DEPLOY_GUIDE.md)
- [数据库迁移问题](database-migration-guide.md)
- [SQLx编译问题](sqlx-compilation-issues.md)
- [邮件配置指南](../user/guides/QUICK_EMAIL_SETUP.md)