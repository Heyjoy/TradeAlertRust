# 邮件通知功能设置指南

## 概述

交易预警系统现在支持邮件通知功能，当价格触发预警条件时，系统会自动发送邮件通知。

## 功能特性

- SMTP邮件发送
- 精美的HTML邮件模板
- 测试邮件功能
- 预警触发时自动发送通知

## 配置步骤

### 1. 更新配置文件

在 `config.toml` 文件中配置邮件设置：

```toml
[email]
# SMTP服务器配置
smtp_server = "smtp.gmail.com"
smtp_port = 587
smtp_username = "your_email@gmail.com"
smtp_password = "your_app_password"
from_email = "your_email@gmail.com"
from_name = "交易预警系统"
# 接收预警的邮箱
to_email = "your_email@gmail.com"
# 是否启用邮件通知
enabled = true
```

### 2. Gmail 配置（推荐）

如果使用Gmail作为SMTP服务器：

1. 启用两步验证
2. 生成应用专用密码：
   - 进入 Google 账户设置
   - 安全 → 两步验证 → 应用专用密码
   - 选择"邮件"和设备
   - 复制生成的16位密码

3. 在配置文件中使用：
   ```toml
   smtp_server = "smtp.gmail.com"
   smtp_port = 587
   smtp_username = "your_email@gmail.com"
   smtp_password = "your_16_digit_app_password"
   ```

### 3. 其他邮件服务商配置

#### QQ邮箱
```toml
smtp_server = "smtp.qq.com"
smtp_port = 587
smtp_username = "your_email@qq.com"
smtp_password = "your_authorization_code"
```

#### 163邮箱
```toml
smtp_server = "smtp.163.com"
smtp_port = 587
smtp_username = "your_email@163.com"
smtp_password = "your_authorization_code"
```

#### Outlook
```toml
smtp_server = "smtp-mail.outlook.com"
smtp_port = 587
smtp_username = "your_email@outlook.com"
smtp_password = "your_password"
```

## 测试邮件功能

### 1. 启动服务
```bash
cargo run
```

### 2. 发送测试邮件
使用curl或浏览器访问：
```bash
curl http://localhost:3000/api/test-email
```

或者在浏览器中访问：
```
http://localhost:3000/api/test-email
```

### 3. 检查响应
成功响应：
```json
{
  "success": true,
  "message": "测试邮件发送成功，请检查您的邮箱"
}
```

失败响应：
```json
{
  "success": false,
  "message": "测试邮件发送失败: 错误详情"
}
```

## 邮件模板

### 测试邮件模板
- 清晰的成功确认
- 系统配置信息
- 美观的HTML格式

### 预警邮件模板
- 突出的预警标题
- 详细的价格信息
- 预警条件和触发时间
- 专业的视觉设计

## 常见问题

### 1. 邮件发送失败
- 检查网络连接
- 确认SMTP服务器地址和端口
- 验证用户名和密码
- 确认邮件服务商是否允许SMTP访问

### 2. Gmail 相关问题
- 确保启用了两步验证
- 使用应用专用密码，不要使用账户密码
- 检查Google账户安全设置

### 3. 邮件被拦截
- 检查垃圾邮件箱
- 添加发送地址到白名单
- 确认邮件服务商的安全策略

## 安全建议

1. **不要在代码中硬编码密码**
2. **使用环境变量存储敏感信息**
3. **定期更换应用专用密码**
4. **启用邮件服务商的安全功能**

## 下一步计划

- [ ] 支持多个收件人
- [ ] 自定义邮件模板
- [ ] 邮件发送记录
- [ ] 预警频率控制 