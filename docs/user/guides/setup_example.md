# 快速设置示例

## 第一次使用？按照以下步骤快速配置

### 1. 复制配置模板

```bash
# Windows
copy config.toml.example config.local.toml

# Linux/macOS  
cp config.toml.example config.local.toml
```

### 2. 编辑本地配置

打开 `config.local.toml` 文件，修改邮件配置：

```toml
[email]
smtp_server = "smtp.gmail.com"
smtp_port = 587
smtp_username = "你的邮箱@gmail.com"           # 替换为您的真实邮箱
smtp_password = "您的应用专用密码"              # 替换为Gmail应用专用密码
from_email = "你的邮箱@gmail.com"             # 发件人邮箱
from_name = "交易预警系统"
to_email = "你的邮箱@gmail.com"               # 接收通知的邮箱
enabled = true
```

### 3. 或者使用 .env 文件（推荐）

创建 `.env` 文件：

```bash
TRADE_ALERT_EMAIL_SMTP_USERNAME=你的邮箱@gmail.com
TRADE_ALERT_EMAIL_SMTP_PASSWORD=您的应用专用密码
TRADE_ALERT_EMAIL_FROM_EMAIL=你的邮箱@gmail.com  
TRADE_ALERT_EMAIL_TO_EMAIL=你的邮箱@gmail.com
```

### 4. 获取Gmail应用专用密码

1. 登录Gmail，进入 [Google账户管理](https://myaccount.google.com/)
2. 选择 "安全性" → "两步验证"（必须先启用）
3. 选择 "应用专用密码"
4. 选择应用类型为 "邮件"，设备类型为 "其他"
5. 输入名称如 "交易预警系统"
6. 复制生成的16位密码

### 5. 启动并测试

```bash
# 启动服务
cargo run

# 测试邮件功能
.\test_email.ps1
```

### 其他邮件服务商

#### QQ邮箱
```toml
smtp_server = "smtp.qq.com"
smtp_port = 587
smtp_username = "你的QQ号@qq.com"
smtp_password = "授权码"  # 需要在QQ邮箱设置中获取
```

#### 163邮箱
```toml
smtp_server = "smtp.163.com"  
smtp_port = 587
smtp_username = "你的邮箱@163.com"
smtp_password = "授权码"  # 需要在163邮箱设置中获取
```

### 安全提醒

- ✅ 使用 `config.local.toml` 或 `.env` (已被.gitignore排除)
- ✅ 使用应用专用密码，不要使用账户密码
- ❌ 不要将真实配置提交到Git
- ❌ 不要在 `config.toml` 中存放真实密码

完成后，您就可以开始使用交易预警系统了！🎉 