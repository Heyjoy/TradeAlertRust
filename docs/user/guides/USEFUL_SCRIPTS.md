# 🚀 TradeAlert 有用脚本说明

## 📋 保留的有用脚本

### 本地运行脚本
- **`run_server.bat`** - 在Windows本地运行TradeAlert服务器
  - 双击即可运行
  - 访问 http://localhost:8000
  - 适合开发和测试

### PowerShell脚本
- **`start.ps1`** - PowerShell版本的启动脚本
- **`start_public.ps1`** - 公网访问版本的启动脚本（包含ngrok配置）
- **`test_email.ps1`** - 邮件功能测试脚本
- **`test_yahoo_api.ps1`** - Yahoo财经API测试脚本
- **`test_api.ps1`** - 通用API测试脚本

### 批处理脚本
- **`start.bat`** - 简单启动脚本
- **`start_public.bat`** - 公网访问启动脚本

## 🎯 推荐使用方案

### 方案1：本地运行（最简单）
```bash
双击 run_server.bat
```
- ✅ 无需复杂配置
- ✅ 立即可用
- ❌ 仅限本地访问

### 方案2：公网访问
```bash
双击 start_public.bat
```
- ✅ 支持远程访问
- ✅ 包含ngrok隧道
- ❌ 需要ngrok配置

### 方案3：NAS部署
由于ARM交叉编译复杂，建议：
- 🍓 使用树莓派（300元，原生ARM环境）
- ☁️ 使用云服务器（9元/月起）
- 🖥️ 使用本地运行 + 内网穿透

## 📁 其他有用文件

- **`config.toml.example`** - 配置文件示例
- **`setup_example.md`** - 设置示例说明
- **`friend_test_guide.md`** - 朋友测试指南
- **`README.md`** - 项目主要说明

## 💡 清理说明

已删除的复杂部署脚本：
- ❌ 各种NAS部署脚本（权限和交叉编译问题）
- ❌ SSH密钥设置脚本（过于复杂）
- ❌ Cross编译脚本（Windows环境不友好）

保留的都是**简单可用**的脚本，建议优先使用本地运行方案。 