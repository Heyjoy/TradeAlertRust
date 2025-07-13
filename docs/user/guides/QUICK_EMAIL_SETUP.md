# 🚀 快速邮箱测试设置 (5分钟搞定)

## Gmail设置 (推荐)

### 1. 启用两步验证
1. 访问 [Google账户安全设置](https://myaccount.google.com/security)
2. 启用"两步验证"

### 2. 生成应用专用密码
1. 在两步验证下找到"应用专用密码"
2. 选择"邮件"应用类型
3. 复制生成的16位密码 (例如: `abcd efgh ijkl mnop`)

### 3. 编辑配置文件
编辑 `config.local.toml`:
```toml
[email]
smtp_username = "你的邮箱@gmail.com"
smtp_password = "abcd efgh ijkl mnop"  # 刚才生成的应用专用密码
from_email = "你的邮箱@gmail.com"
to_email = "你的邮箱@gmail.com"      # 可以发给自己测试
```

## 其他邮箱选择

### QQ邮箱
```toml
[email]
smtp_server = "smtp.qq.com"
smtp_port = 587
smtp_username = "你的QQ号@qq.com"
smtp_password = "你的QQ邮箱授权码"  # 不是QQ密码！
```

### 163邮箱
```toml
[email]
smtp_server = "smtp.163.com"
smtp_port = 587
smtp_username = "你的邮箱@163.com"
smtp_password = "你的邮箱授权码"   # 不是登录密码！
```

## 🧪 测试步骤

1. **配置邮箱** (3分钟)
2. **启动服务** (1分钟)
   ```bash
   cargo run
   ```
3. **测试邮件** (1分钟)
   ```bash
   curl http://localhost:3000/api/test-email
   ```
4. **创建预警测试** - 设置一个容易触发的价格条件
5. **等待邮件** - 系统会自动发送预警邮件

## ✅ 成功标志
- 收到测试邮件 ✓
- 收到预警触发邮件 ✓
- 系统正常运行 ✓

## 🎯 测试完成后
确认系统完全工作，再考虑：
- 专业域名 (可选)
- 生产环境部署
- 更多功能扩展

**现在就开始测试吧！** 🚀 