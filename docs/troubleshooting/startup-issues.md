# 🚀 启动和运行问题解决方案

## 目录
- [端口占用问题](#端口占用问题)
- [服务器启动失败](#服务器启动失败)
- [环境变量配置](#环境变量配置)
- [数据库连接问题](#数据库连接问题)

---

## 端口占用问题 {#port-occupied}

### 问题症状
```
Error: 通常每个套接字地址(协议/网络地址/端口)只允许使用一次。 (os error 10048)
error: process didn't exit successfully: `target\debug\trade_alert_rust.exe` (exit code: 1)
```

### 原因分析
- 端口3000已被其他进程占用
- 之前的TradeAlert进程未正确关闭
- 其他应用程序正在使用该端口

### 🔧 解决方案

#### 方案1: 快速杀死占用进程（推荐）
```powershell
# 查找占用3000端口的进程
netstat -ano | findstr :3000

# 杀死进程（替换PID为实际进程ID）
taskkill /PID <进程ID> /F

# 或者直接杀死所有相关进程
taskkill /f /im trade_alert_rust.exe
taskkill /f /im cargo.exe
```

#### 方案2: 更换端口
```powershell
# 临时使用其他端口
$env:TRADE_ALERT__SERVER__PORT="3001"
cargo run

# 或修改配置文件
# config/config.toml
[server]
port = 3001
```

#### 方案3: 系统重启（最后手段）
```powershell
# 如果进程无法杀死，重启系统
shutdown /r /t 0
```

### 预防措施
- 使用 `Ctrl+C` 正确关闭服务器
- 定期检查和清理僵尸进程
- 使用进程管理工具

---

## 服务器启动失败 {#server-startup-failure}

### 问题症状
- 服务器无法启动
- 编译错误
- 依赖缺失

### 🔧 解决方案

#### 检查环境变量
```powershell
# 确保必要的环境变量已设置
$env:SQLX_OFFLINE="false"
$env:DATABASE_URL="sqlite:data/trade_alert.db"
$env:RUST_LOG="info"

# 验证环境变量
echo $env:SQLX_OFFLINE
echo $env:DATABASE_URL
```

#### 清理和重建
```powershell
# 清理构建缓存
cargo clean

# 重新构建
cargo build

# 运行迁移
cargo run --bin migrate

# 启动服务器
cargo run
```

#### 检查依赖
```powershell
# 更新依赖
cargo update

# 检查Cargo.toml语法
cargo check
```

---

## 环境变量配置 {#environment-config}

### 必需的环境变量

#### 数据库配置
```powershell
$env:DATABASE_URL="sqlite:data/trade_alert.db"
$env:SQLX_OFFLINE="false"
```

#### 服务器配置
```powershell
$env:TRADE_ALERT__SERVER__HOST="127.0.0.1"
$env:TRADE_ALERT__SERVER__PORT="3000"
```

#### 邮件配置
```powershell
$env:TRADE_ALERT__EMAIL__ENABLED="true"
$env:TRADE_ALERT__EMAIL__SMTP_SERVER="smtp.gmail.com"
$env:TRADE_ALERT__EMAIL__SMTP_PORT="587"
$env:TRADE_ALERT__EMAIL__SMTP_USERNAME="your-email@gmail.com"
$env:TRADE_ALERT__EMAIL__SMTP_PASSWORD="your-app-password"
$env:TRADE_ALERT__EMAIL__FROM_EMAIL="your-email@gmail.com"
$env:TRADE_ALERT__EMAIL__FROM_NAME="TradeAlert"
$env:TRADE_ALERT__EMAIL__TO_EMAIL="your-email@gmail.com"
```

### 配置文件方式
创建 `config/config.toml`:
```toml
[server]
host = "127.0.0.1"
port = 3000

[database]
url = "sqlite:data/trade_alert.db"

[email]
enabled = true
smtp_server = "smtp.gmail.com"
smtp_port = 587
smtp_username = "your-email@gmail.com"
smtp_password = "your-app-password"
from_email = "your-email@gmail.com"
from_name = "TradeAlert"
to_email = "your-email@gmail.com"

[logging]
level = "info"

[price_fetcher]
update_interval_secs = 30

[scheduler]
default_schedule = "*/5 * * * *"
```

---

## 数据库连接问题 {#database-connection}

### 问题症状
- SQLite文件不存在
- 数据库锁定
- 迁移失败

### 🔧 解决方案

#### 创建数据库目录
```powershell
# 确保数据目录存在
mkdir -p data

# 检查数据库文件
ls data/
```

#### 重新运行迁移
```powershell
# 删除现有数据库（注意：会丢失数据）
rm data/trade_alert.db

# 重新运行迁移
cargo run --bin migrate
```

#### 检查数据库内容
```powershell
# 使用SQLite命令行工具
sqlite3 data/trade_alert.db

# 查看表结构
.tables
.schema alerts

# 退出
.quit
```

---

## 完整重置环境 {#reset-environment}

### 🚨 紧急重置流程

```powershell
# 1. 停止所有相关进程
taskkill /f /im trade_alert_rust.exe 2>$null
taskkill /f /im cargo.exe 2>$null

# 2. 清理构建产物
cargo clean
rm -rf target/

# 3. 重置数据库
rm data/trade_alert.db

# 4. 设置环境变量
$env:SQLX_OFFLINE="false"
$env:DATABASE_URL="sqlite:data/trade_alert.db"
$env:RUST_LOG="info"

# 5. 重新构建
cargo build

# 6. 运行迁移
cargo run --bin migrate

# 7. 启动服务器
cargo run
```

---

## 诊断脚本

### 创建自动诊断脚本
```powershell
# scripts/diagnose-startup.ps1
Write-Host "🔍 TradeAlert 启动诊断" -ForegroundColor Cyan

# 检查端口占用
Write-Host "`n📡 检查端口占用..."
$port3000 = netstat -ano | findstr :3000
if ($port3000) {
    Write-Host "❌ 端口3000被占用: $port3000" -ForegroundColor Red
} else {
    Write-Host "✅ 端口3000可用" -ForegroundColor Green
}

# 检查环境变量
Write-Host "`n🔧 检查环境变量..."
$envVars = @("SQLX_OFFLINE", "DATABASE_URL", "RUST_LOG")
foreach ($var in $envVars) {
    $value = [Environment]::GetEnvironmentVariable($var)
    if ($value) {
        Write-Host "✅ $var = $value" -ForegroundColor Green
    } else {
        Write-Host "❌ $var 未设置" -ForegroundColor Red
    }
}

# 检查数据库文件
Write-Host "`n💾 检查数据库..."
if (Test-Path "data/trade_alert.db") {
    Write-Host "✅ 数据库文件存在" -ForegroundColor Green
} else {
    Write-Host "❌ 数据库文件不存在" -ForegroundColor Red
}

# 检查编译状态
Write-Host "`n🔨 检查编译状态..."
$compileResult = cargo check 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ 编译通过" -ForegroundColor Green
} else {
    Write-Host "❌ 编译失败" -ForegroundColor Red
    Write-Host $compileResult
}

Write-Host "`n🎯 诊断完成！" -ForegroundColor Cyan
```

---

## 快速解决清单

- [ ] 检查端口占用 (`netstat -ano | findstr :3000`)
- [ ] 杀死占用进程 (`taskkill /f /im trade_alert_rust.exe`)
- [ ] 设置环境变量 (`$env:SQLX_OFFLINE="false"`)
- [ ] 清理重建 (`cargo clean && cargo build`)
- [ ] 运行迁移 (`cargo run --bin migrate`)
- [ ] 启动服务器 (`cargo run`)
- [ ] 验证访问 (http://localhost:3000)

**预估解决时间**: 2-5分钟  
**难度等级**: 🟢 简单 