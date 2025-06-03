# 交易预警系统 (Rust版本)

一个基于Rust开发的股票价格监控和预警系统，支持实时价格获取、预警设置和邮件通知。

## 主要功能

### ✅ 已完成功能

1. **Web界面和API**
   - 现代化的Web界面
   - 完整的REST API
   - 预警管理功能

2. **数据库存储**
   - SQLite数据库
   - 预警数据持久化
   - 价格历史记录

3. **股票价格获取** 
   - Yahoo Finance API集成
   - 自动价格更新
   - 并发请求控制
   - 错误重试机制

4. **📧 邮件通知系统**
   - SMTP邮件发送
   - 精美的HTML邮件模板
   - 测试邮件功能
   - 支持多种邮件服务商

### 🚧 开发中功能

5. **实时预警监控** (下一步)
   - 自动价格检查
   - 预警触发检测
   - 邮件通知发送

## 快速开始

### 1. 安装依赖

确保已安装Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 配置邮件服务

编辑 `config.toml` 文件：

```toml
[email]
smtp_server = "smtp.gmail.com"
smtp_port = 587
smtp_username = "your_email@gmail.com"
smtp_password = "your_app_password"  # Gmail应用专用密码
from_email = "your_email@gmail.com"
from_name = "交易预警系统"
to_email = "your_email@gmail.com"
enabled = true
```

**重要**: 
- Gmail用户需要启用两步验证并生成应用专用密码
- 详细配置说明请查看 `docs/email-setup.md`

### 3. 启动服务

```bash
cargo run
```

服务将在 http://localhost:3000 启动

### 4. 测试邮件功能

运行测试脚本:
```powershell
.\test_email.ps1
```

或手动测试:
```bash
curl http://localhost:3000/api/test-email
```

## API端点

### 预警管理
- `GET /api/alerts` - 获取所有预警
- `POST /api/alerts` - 创建新预警
- `GET /api/alerts/{id}` - 获取特定预警
- `PUT /api/alerts/{id}` - 更新预警
- `DELETE /api/alerts/{id}` - 删除预警

### 价格数据
- `GET /api/prices/{symbol}/latest` - 获取最新价格
- `GET /api/prices/{symbol}` - 获取价格历史

### 邮件通知
- `GET /api/test-email` - 发送测试邮件

## 项目结构

```
src/
├── main.rs          # 主程序入口
├── config.rs        # 配置管理
├── db.rs           # 数据库操作
├── models.rs       # 数据模型
├── fetcher.rs      # 价格获取服务
├── email.rs        # 邮件通知模块
└── templates/      # HTML模板
```

## 支持的邮件服务商

- **Gmail** (推荐)
- **QQ邮箱**
- **163邮箱** 
- **Outlook**
- 其他支持SMTP的邮件服务

## 开发路线图

### 第一阶段 ✅
- [x] 基本Web界面
- [x] 数据库设计
- [x] 价格获取API
- [x] 邮件通知系统

### 第二阶段 🚧
- [ ] 实时预警监控
- [ ] 预警触发逻辑
- [ ] 自动邮件发送

### 第三阶段 📅
- [ ] 更多预警条件
- [ ] 历史数据分析
- [ ] 性能优化

## 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交变更
4. 推送到分支
5. 创建Pull Request

## 许可证

MIT License 