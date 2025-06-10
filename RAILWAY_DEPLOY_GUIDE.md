# Railway 部署指南

## 🚀 快速部署步骤

### 1. 准备工作
确保代码已推送到GitHub仓库

### 2. 创建Railway项目
1. 访问 [Railway.app](https://railway.app)
2. 注册/登录账号
3. 点击 "Deploy a new project"
4. 选择 "Deploy from GitHub repo"
5. 选择你的仓库

### 3. 配置环境变量
在Railway Dashboard中，进入你的服务 → Variables 标签页，添加以下环境变量：

#### 必需的邮件配置
```
TRADE_ALERT_EMAIL_SMTP_SERVER=smtp.gmail.com
TRADE_ALERT_EMAIL_SMTP_PORT=587
TRADE_ALERT_EMAIL_SMTP_USERNAME=你的Gmail地址
TRADE_ALERT_EMAIL_SMTP_PASSWORD=你的Gmail应用专用密码
TRADE_ALERT_EMAIL_FROM_EMAIL=你的Gmail地址
TRADE_ALERT_EMAIL_FROM_NAME=股票预警系统
TRADE_ALERT_EMAIL_TO_EMAIL=你的Gmail地址
TRADE_ALERT_EMAIL_ENABLED=true
```

#### 可选配置
```
TRADE_ALERT_LOGGING_LEVEL=info
TRADE_ALERT_SCHEDULER_DEFAULT_SCHEDULE=*/5 * * * *
```

### 4. 部署
1. Railway会自动检测到Rust项目
2. 第一次构建可能需要几分钟
3. 构建完成后会自动部署

### 5. 访问应用
1. 在Railway Dashboard中点击你的服务
2. 点击 "Generate Domain" 获取公网URL
3. 访问URL测试应用

## 📋 环境变量详解

### 邮件配置说明
- `SMTP_USERNAME` & `FROM_EMAIL`: 你的Gmail地址
- `SMTP_PASSWORD`: Gmail的应用专用密码（不是登录密码）
  - 开启Gmail两步验证
  - 生成应用专用密码：https://myaccount.google.com/apppasswords
- `TO_EMAIL`: 默认接收邮件的地址

### 如何获取Gmail应用专用密码
1. 登录Gmail
2. 进入 Google账户设置
3. 安全性 → 两步验证（必须开启）
4. 应用专用密码 → 生成密码
5. 复制16位密码（格式：xxxx xxxx xxxx xxxx）

## 🔧 故障排除

### 构建失败
- 检查Cargo.toml是否正确
- 查看Railway构建日志

### 应用启动失败
- 检查端口配置（Railway会自动设置PORT环境变量）
- 检查日志中的错误信息

### 邮件发送失败
- 确认Gmail应用专用密码正确
- 检查环境变量设置
- 查看应用日志

### 数据库问题
- SQLite文件会在每次部署时重置
- 生产环境建议使用Railway的PostgreSQL

## 🎯 下一步优化

1. **自定义域名**: 在Railway中配置你的域名
2. **PostgreSQL**: 添加Railway PostgreSQL数据库
3. **监控**: 使用Railway的监控功能
4. **扩展**: 配置多副本部署

## 💡 提示
- Railway提供$5/月的免费额度
- 首次部署可能较慢，后续会更快
- 支持自动部署：推送到GitHub会自动触发部署 