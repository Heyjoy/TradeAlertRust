# 交易预警系统 (Rust版本)

一个基于Rust开发的股票价格监控和预警系统，支持实时价格获取、预警设置和邮件通知。

## 主要功能

### ✅ 已完成功能

1. **Web界面和API**
   - 现代化的Web界面
   - 完整的REST API
   - 预警管理功能 (CRUD操作)

2. **数据库存储**
   - SQLite数据库
   - 预警数据持久化
   - 价格历史记录

3. **股票价格获取** 
   - Yahoo Finance API集成
   - 自动价格更新 (30秒间隔)
   - 并发请求控制
   - 错误重试机制
   - 缓存机制

4. **📧 邮件通知系统**
   - SMTP邮件发送基础设施
   - 精美的HTML邮件模板
   - 测试邮件功能
   - 支持多种邮件服务商

5. **🔄 实时价格监控**
   - 后台价格更新服务
   - 智能缓存管理
   - 请求频率控制

### 🚧 当前状态与问题

6. **⚠️ 预警触发系统** - **关键集成缺失**
   - ✅ 预警条件检测逻辑已实现
   - ✅ 数据库状态更新正常
   - ❌ **邮件通知未集成** - 预警触发时不发送邮件
   - ❌ 配置文件使用占位符，需要真实邮件凭据

### 🎯 下一步重点 (今日任务)

**核心问题：** 实时预警监控系统 **90%完成**，但缺少最关键的邮件通知集成

#### 紧急修复
1. **🔗 集成邮件通知到预警触发**
   ```rust
   // 需要在 src/fetcher.rs 的 check_alerts() 中添加邮件发送
   if triggered {
       // 现有代码：更新数据库状态 ✅
       self.mark_alert_triggered(alert_id).await?;
       // 缺失代码：发送邮件通知 ❌
       // self.email_notifier.send_alert_notification(&alert, current_price).await?;
   }
   ```

2. **⚙️ 邮件配置设置**
   - 配置真实的Gmail SMTP凭据
   - 测试完整预警流程

3. **🧹 代码清理**
   - 修复编译警告
   - 移除未使用的代码

### 🚀 完成后的效果
- 价格达到预警条件时自动发送邮件 📧
- 完整的端到端预警系统 🎯
- 真正的"实时交易预警" ⚡

### 🔄 开发中功能

7. **系统优化** (后续)
   - 更多预警条件类型
   - 批量邮件发送
   - 性能监控面板

## 快速开始

### 1. 安装依赖

确保已安装Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 配置邮件服务

**🔒 重要安全提醒：** 为保护隐私，请勿将邮箱密码等敏感信息上传到GitHub！

#### 方法一：环境变量配置（推荐）

创建 `.env` 文件：

```bash
# .env 文件 (已被.gitignore排除)
TRADE_ALERT_EMAIL_SMTP_USERNAME=your_email@gmail.com
TRADE_ALERT_EMAIL_SMTP_PASSWORD=your_app_password
TRADE_ALERT_EMAIL_FROM_EMAIL=your_email@gmail.com
TRADE_ALERT_EMAIL_TO_EMAIL=your_email@gmail.com
```

#### 方法二：本地配置文件

```bash
# 复制配置模板
cp config.toml.example config.local.toml
# 编辑 config.local.toml 填入真实配置
```

#### 方法三：系统环境变量

```bash
export TRADE_ALERT_EMAIL_SMTP_USERNAME="your_email@gmail.com"
export TRADE_ALERT_EMAIL_SMTP_PASSWORD="your_app_password"
```

**重要**: 
- Gmail用户需要启用两步验证并生成应用专用密码
- 详细配置说明请查看 `docs/security-config.md`
- 邮件设置指南请查看 `docs/email-setup.md`

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

### 第一阶段 ✅ **已完成**
- [x] 基本Web界面和API
- [x] 数据库设计和操作
- [x] Yahoo Finance价格获取
- [x] 邮件通知基础设施
- [x] 实时价格监控服务

### 第二阶段 🚧 **进行中 (90%完成)**
- [x] 预警条件检测逻辑
- [x] 数据库状态自动更新  
- [ ] **邮件通知集成** ⚠️ 关键缺失
- [ ] 完整端到端测试

### 第三阶段 📅 **计划中**
- [ ] 更多预警条件类型 (百分比变化、移动平均等)
- [ ] 批量邮件管理
- [ ] 历史数据分析和图表
- [ ] 性能优化和监控面板
- [ ] WebSocket实时推送

### 🎯 **当前优先级**
1. **紧急** - 修复邮件通知集成 (1-2小时)
2. **重要** - 邮件配置和测试 (30分钟)  
3. **优化** - 代码清理和警告修复 (30分钟)

## 贡献指南

1. Fork 项目
2. 创建功能分支
3. 提交变更
4. 推送到分支
5. 创建Pull Request

## 许可证

MIT License 