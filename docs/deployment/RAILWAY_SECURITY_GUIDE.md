# Railway 安全配置指南

## 🔒 安全性说明

Railway在安全方面采用了多层保护：
- **加密存储**: 环境变量在服务器端加密存储
- **传输加密**: 所有API调用使用HTTPS/TLS
- **访问控制**: 基于项目的权限管理
- **Sealed Variables**: 设置后连管理员都无法查看

## 🛡️ 推荐的安全配置方法

### 1. 使用Sealed Variables（推荐）

对于敏感信息如Gmail密码，使用Railway的Sealed Variables功能：

#### 操作步骤：
1. 在Railway Dashboard进入你的服务
2. 点击 Variables 标签页
3. 添加变量时，点击变量右侧的"⋯"菜单
4. 选择 "Seal" 选项
5. 设置后该变量值将无法查看

#### 建议Seal的变量：
```
TRADE_ALERT_EMAIL_SMTP_PASSWORD  ← 一定要Seal
TRADE_ALERT_EMAIL_SMTP_USERNAME  ← 建议Seal  
```

### 2. 环境变量命名最佳实践

使用描述性但不暴露敏感信息的名称：
```bash
# ✅ 好的命名
TRADE_ALERT_EMAIL_SMTP_PASSWORD=xxxxx

# ❌ 避免的命名  
GMAIL_PASSWORD=xxxxx
PERSONAL_EMAIL_PASS=xxxxx
```

### 3. 分层配置策略

```bash
# 🔓 可以公开的配置
TRADE_ALERT_EMAIL_SMTP_SERVER=smtp.gmail.com
TRADE_ALERT_EMAIL_SMTP_PORT=587
TRADE_ALERT_EMAIL_FROM_NAME=股票预警系统
TRADE_ALERT_EMAIL_ENABLED=true

# 🔒 需要保护的配置（使用Sealed Variables）
TRADE_ALERT_EMAIL_SMTP_USERNAME=xxxxx  # Seal this
TRADE_ALERT_EMAIL_SMTP_PASSWORD=xxxxx  # Seal this
TRADE_ALERT_EMAIL_FROM_EMAIL=xxxxx     # Seal this
TRADE_ALERT_EMAIL_TO_EMAIL=xxxxx       # Seal this
```

## 🚨 安全风险评估

### Railway vs 其他方案

| 存储方式 | 风险等级 | 说明 |
|----------|----------|------|
| `config.local.toml` | ⚠️ 中高 | 本地明文，可能意外共享 |
| `.env` 文件 | ⚠️ 中高 | 容易意外提交到git |
| Railway普通变量 | ✅ 低 | 加密存储，权限控制 |
| Railway Sealed变量 | ✅ 极低 | 设置后不可查看 |

### 潜在风险点

1. **账号泄露风险**
   - 如果Railway账号被入侵
   - 缓解：启用2FA，使用强密码

2. **团队成员风险**  
   - 团队成员可能查看变量
   - 缓解：使用Sealed Variables

3. **日志泄露风险**
   - 应用日志可能意外输出敏感信息
   - 缓解：代码中避免打印敏感变量

## 🔧 增强安全性的额外措施

### 1. 使用专用邮箱账号

```bash
# 推荐：为应用创建专用Gmail账号
SMTP_USERNAME=tradealert.notification@gmail.com

# 而不是使用个人邮箱
# SMTP_USERNAME=personal.email@gmail.com
```

### 2. 限制Gmail应用密码权限

- 只给予"发送邮件"权限
- 定期轮换应用密码
- 监控Gmail安全活动

### 3. 代码中的安全实践

确保应用代码不会泄露敏感信息：
```rust
// ✅ 好的做法
tracing::info!("邮件发送成功");

// ❌ 避免的做法  
tracing::info!("使用密码 {} 发送邮件", password);
```

## 📋 安全检查清单

部署前请确认：

- [ ] 敏感环境变量已设置为Sealed
- [ ] 使用专用邮箱账号（推荐）
- [ ] 启用Railway账号2FA
- [ ] 应用代码不输出敏感信息到日志
- [ ] 定期检查Railway访问日志
- [ ] 考虑定期轮换Gmail应用密码

## 🆚 对比：Railway vs 自建服务器

| 方面 | Railway | 自建服务器 |
|------|---------|------------|
| 基础设施安全 | ✅ 专业团队维护 | ⚠️ 需要自己维护 |
| 环境变量安全 | ✅ 加密存储+Sealed | ⚠️ 取决于配置 |
| 访问控制 | ✅ 内置权限管理 | ⚠️ 需要自己实现 |
| 审计日志 | ✅ 自动记录 | ⚠️ 需要自己配置 |
| 更新维护 | ✅ 自动更新 | ⚠️ 需要手动维护 |

## 💡 结论

Railway的环境变量机制比本地配置文件更安全，特别是使用Sealed Variables功能后。对于个人项目或小团队，Railway提供了足够的安全保障。 