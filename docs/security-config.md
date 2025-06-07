# 安全配置指南

## 概述

为了保护您的隐私和敏感信息，本项目提供了多种安全的配置方式，避免将敏感信息（如邮箱密码、API密钥等）上传到版本控制系统。

## 🚨 重要安全提醒

**绝对不要将以下敏感信息提交到GitHub：**
- 邮箱密码 (smtp_password)
- 邮箱用户名（可能暴露个人信息）
- API密钥
- 数据库连接字符串（如包含密码）

## 配置文件优先级

系统按以下顺序加载配置（后加载的会覆盖前面的）：

1. `config.toml.example` - 公开的配置模板（可以上传到GitHub）
2. `config.local.toml` - 本地配置文件（被.gitignore排除）
3. `config.toml` - 主配置文件（被.gitignore排除）
4. **环境变量** - 最高优先级（推荐用于敏感信息）

## 方法一：环境变量配置（推荐）

### 1. 创建.env文件

在项目根目录创建 `.env` 文件（已被.gitignore排除）：

```bash
# 邮件配置
SMTP_USERNAME=your_email@gmail.com
SMTP_PASSWORD=your_app_password_here
FROM_EMAIL=your_email@gmail.com
TO_EMAIL=your_email@gmail.com

# 或者使用完整的环境变量名（推荐）
TRADE_ALERT_EMAIL_SMTP_USERNAME=your_email@gmail.com
TRADE_ALERT_EMAIL_SMTP_PASSWORD=your_app_password_here
TRADE_ALERT_EMAIL_FROM_EMAIL=your_email@gmail.com
TRADE_ALERT_EMAIL_TO_EMAIL=your_email@gmail.com
```

### 2. 使用dotenvy加载环境变量

在 `main.rs` 中添加：

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载.env文件（如果存在）
    dotenvy::dotenv().ok();
    
    // 其余代码...
}
```

### 3. 环境变量命名规则

格式：`TRADE_ALERT_<SECTION>_<KEY>`

示例：
```bash
TRADE_ALERT_EMAIL_SMTP_USERNAME=user@gmail.com
TRADE_ALERT_EMAIL_SMTP_PASSWORD=app_password
TRADE_ALERT_SERVER_PORT=8080
TRADE_ALERT_LOGGING_LEVEL=debug
```

## 方法二：本地配置文件

### 1. 复制配置模板

```bash
cp config.toml.example config.local.toml
```

### 2. 编辑本地配置

编辑 `config.local.toml`，填入真实的敏感信息：

```toml
[email]
smtp_username = "your_email@gmail.com"
smtp_password = "your_app_password"
from_email = "your_email@gmail.com"
to_email = "your_email@gmail.com"
```

## 方法三：配置模板中的占位符

`config.toml.example` 使用占位符语法：

```toml
[email]
smtp_username = "${SMTP_USERNAME}"
smtp_password = "${SMTP_PASSWORD}"
from_email = "${FROM_EMAIL}"
to_email = "${TO_EMAIL}"
```

系统会自动替换为对应的环境变量值。

## 生产环境部署

### Docker部署

```dockerfile
# Dockerfile中不包含敏感信息
ENV TRADE_ALERT_EMAIL_SMTP_SERVER=smtp.gmail.com
ENV TRADE_ALERT_EMAIL_SMTP_PORT=587
# 敏感信息通过docker run时传入
```

运行时传入敏感信息：

```bash
docker run -e TRADE_ALERT_EMAIL_SMTP_USERNAME=user@gmail.com \
           -e TRADE_ALERT_EMAIL_SMTP_PASSWORD=app_password \
           your-app:latest
```

### 系统环境变量

在Linux/macOS中：

```bash
export TRADE_ALERT_EMAIL_SMTP_USERNAME="your_email@gmail.com"
export TRADE_ALERT_EMAIL_SMTP_PASSWORD="your_app_password"
```

在Windows中：

```cmd
set TRADE_ALERT_EMAIL_SMTP_USERNAME=your_email@gmail.com
set TRADE_ALERT_EMAIL_SMTP_PASSWORD=your_app_password
```

## 文件权限设置

确保敏感配置文件的权限设置正确：

```bash
# 只有所有者可读写
chmod 600 config.local.toml
chmod 600 .env

# 确保其他用户无法访问
ls -la config.local.toml
# 应该显示：-rw------- 1 user user ...
```

## 开发团队协作

### 1. Git配置

确保 `.gitignore` 包含：

```gitignore
# 敏感配置文件
/config.toml
/config.local.toml
/.env

# 数据库文件
/trade_alert.db
```

### 2. 团队成员设置

每个开发者需要：

1. 复制 `config.toml.example` 为 `config.local.toml`
2. 填入自己的配置信息
3. 或者设置环境变量

### 3. CI/CD配置

在GitHub Actions等CI系统中：

```yaml
env:
  TRADE_ALERT_EMAIL_SMTP_USERNAME: ${{ secrets.SMTP_USERNAME }}
  TRADE_ALERT_EMAIL_SMTP_PASSWORD: ${{ secrets.SMTP_PASSWORD }}
```

## 安全检查清单

- [ ] `.gitignore` 包含所有敏感配置文件
- [ ] 使用环境变量存储密码
- [ ] 配置文件权限设置正确
- [ ] 不在代码中硬编码敏感信息
- [ ] 使用应用专用密码而非账户密码
- [ ] 定期更换密码和API密钥
- [ ] 代码提交前检查是否包含敏感信息

## 常用命令

检查是否有敏感信息被意外提交：

```bash
# 检查Git历史中的敏感信息
git log --grep="password" --oneline
git log -p | grep -i "password\|secret\|key"

# 从Git历史中移除敏感文件
git filter-branch --force --index-filter \
  'git rm --cached --ignore-unmatch config.toml' \
  --prune-empty --tag-name-filter cat -- --all
```

## 紧急处理

如果不小心将敏感信息提交到了GitHub：

1. **立即更换密码/密钥**
2. 使用 `git filter-branch` 清理历史
3. 强制推送：`git push origin --force --all`
4. 通知GitHub Support（如果是公开仓库）

记住：**一旦信息上传到互联网，就认为它已经被泄露了！** 