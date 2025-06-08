# 邮件功能问题排查指南

## 问题描述
如果遇到邮件发送失败的错误：
```
Connection error: 由于连接方在一段时间后没有正确答复或连接的主机没有反应，连接尝试失败。 (os error 10060)
```

## 原因分析
这个错误通常是网络连接问题，而不是代码问题。

## 解决方案

### 1. 检查网络连接
```bash
# 运行网络诊断工具
cargo run --bin test_network
```

### 2. 防火墙设置
- **Windows Defender 防火墙**：允许程序通过防火墙
- **第三方防火墙**：添加 Rust 程序到白名单
- **路由器防火墙**：检查是否阻止了 SMTP 端口 (587, 465, 25)

### 3. 网络环境限制
某些网络环境可能会阻止 SMTP 连接：
- **公司网络**：联系 IT 部门
- **学校网络**：联系网络管理员  
- **公共 WiFi**：尝试使用移动热点
- **ISP 限制**：联系互联网服务提供商

### 4. 使用 VPN
如果网络被限制，可以尝试：
- 连接 VPN 后再测试
- 使用不同的网络环境

### 5. 替代邮件服务器
如果 Gmail 不可用，可以尝试其他邮件服务：

#### QQ 邮箱配置
```toml
[email]
smtp_server = "smtp.qq.com"
smtp_port = 587
smtp_username = "your_qq@qq.com"
smtp_password = "your_qq_authorization_code"  # QQ邮箱授权码
from_email = "your_qq@qq.com"
to_email = "your_qq@qq.com"
from_name = "股票预警系统"
enabled = true
```

#### 163 邮箱配置
```toml
[email]
smtp_server = "smtp.163.com"
smtp_port = 465
smtp_username = "your_email@163.com"
smtp_password = "your_163_authorization_code"  # 163邮箱授权码
from_email = "your_email@163.com"
to_email = "your_email@163.com"
from_name = "股票预警系统"
enabled = true
```

#### Outlook 配置
```toml
[email]
smtp_server = "smtp.office365.com"
smtp_port = 587
smtp_username = "your_email@outlook.com"
smtp_password = "your_outlook_password"
from_email = "your_email@outlook.com"
to_email = "your_email@outlook.com"
from_name = "股票预警系统"
enabled = true
```

### 6. 临时禁用邮件功能
如果暂时无法解决网络问题，可以禁用邮件功能：

```toml
[email]
enabled = false
# ... 其他配置保持不变
```

系统将继续正常工作，只是不会发送邮件通知。

### 7. 测试步骤
1. 首先运行网络诊断：`cargo run --bin test_network`
2. 如果网络正常，测试邮件：`cargo run --bin test_email`
3. 如果仍有问题，尝试不同的邮件服务器
4. 最后运行主程序：`cargo run --bin trade_alert_rust`

### 8. 成功标志
当邮件功能正常工作时，你会看到：
```
✅ 测试邮件发送成功！
INFO trade_alert_rust::email: 邮件发送成功: 交易预警系统 - 测试邮件
```

## 常见错误代码
- `os error 10060`: 连接超时（网络问题）
- `os error 10061`: 连接被拒绝（端口被阻止）
- `Authentication failed`: 用户名/密码错误
- `Invalid credentials`: 需要应用专用密码

## 获取帮助
如果问题仍然存在，请检查：
1. 网络管理员是否限制了 SMTP 连接
2. 是否需要使用企业邮件服务器
3. 是否可以使用其他通知方式（如 Webhook、钉钉等） 