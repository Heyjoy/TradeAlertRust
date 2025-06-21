# 交易预警系统开发计划

## 项目概述
开发一个智能市场异动监控系统，帮助投资者捕捉人工监控容易错过的市场机会和风险。

## 产品定位
**核心价值**：帮助投资者捕捉人工监控容易错过的市场机会和风险
**产品边界**：智能监控 + 及时提醒，用户自主决策和操作
**目标用户**：有投资经验但监控时间有限的个人投资者

## 技术栈
- **后端**: Rust + Axum
- **数据库**: SQLite (后期可迁移到PostgreSQL)
- **前端**: HTML5 + CSS3 + JavaScript + Bootstrap
- **模板引擎**: Askama
- **定时任务**: tokio-cron-scheduler
- **邮件系统**: lettre + SMTP
- **行情数据源**: Yahoo Finance API
- **测试工具**: 多个独立测试二进制文件

## 当前状态 (2025-06-14)

### ✅ 已完成功能

#### 1. 核心架构
- [x] 项目框架搭建 (Axum)
- [x] 配置系统实现 (config.toml + 环境变量)
- [x] 数据库迁移系统 (SQLx)
- [x] 模块化架构设计
- [x] SQLx 离线模式支持

#### 2. 数据库层
- [x] 完整的数据库设计
  - alerts 表 (预警记录)
  - alert_history 表 (触发历史)
- [x] 数据库操作封装
- [x] CRUD 操作完整实现

#### 3. API层
- [x] RESTful API 完整实现
  - GET /api/alerts - 获取预警列表
  - POST /api/alerts - 创建预警
  - GET /api/alerts/:id - 获取单个预警
  - DELETE /api/alerts/:id - 删除预警
  - GET /api/test-email - 测试邮件发送
- [x] 错误处理机制
- [x] JSON 响应格式统一

#### 4. 邮件通知系统
- [x] SMTP 邮件发送功能
- [x] 支持多种邮件服务商 (Gmail, QQ, 163, Outlook)
- [x] 智能连接类型识别 (STARTTLS/TLS)
- [x] HTML 邮件模板
- [x] 预警触发邮件通知

#### 5. 价格监控系统
- [x] Yahoo Finance API 集成
- [x] 定时价格检查 (30秒间隔)
- [x] 预警条件判断逻辑
- [x] 自动预警触发
- [x] 价格数据缓存机制

#### 6. Web界面
- [x] 响应式设计 (Bootstrap)
- [x] 预警列表页面
- [x] 预警创建表单
- [x] 预警编辑功能
- [x] 删除确认对话框
- [x] 实时状态显示

#### 7. 测试和诊断工具
- [x] 独立邮件测试 (`test_email.rs`)
- [x] 网络连接诊断 (`test_network.rs`)
- [x] Google服务测试 (`test_google.rs`)
- [x] Yahoo Finance API测试 (`test_yahoo.rs`)
- [x] API测试脚本 (PowerShell)
- [x] 一键测试脚本 (`test_email.ps1`)

#### 8. 个性化邮箱系统
- [x] 数据库迁移添加 `notification_email` 字段
- [x] 预警表单支持自定义邮箱输入
- [x] 邮件发送优先级：个人邮箱 > 默认邮箱
- [x] 完全向后兼容现有预警

#### 9. 实时股票代码验证
- [x] 输入时自动验证股票代码有效性
- [x] 实时显示当前股票价格
- [x] 智能防抖机制 (500ms)
- [x] 美观的状态指示器
- [x] 智能按钮控制
- [x] 网络异常友好处理

#### 10. 公网部署和分享
- [x] ngrok 集成实现公网访问
- [x] 一键启动脚本 (`start_public.ps1`)
- [x] 自动获取和复制公网地址
- [x] 朋友测试指南文档

#### 11. Docker支持
- [x] Dockerfile 配置
- [x] Docker Compose 配置
- [x] DSM 6.2.4 兼容版本
- [x] Redis 缓存支持
- [x] 健康检查配置
- [x] 环境变量配置
- [x] 数据持久化

#### 12. 市场异动监控系统基础架构 (2025-06-14)
- [x] 数据库迁移系统完善
  - [x] 修复应用代码与数据库表结构不匹配问题
  - [x] 创建市场异动监控相关数据表
    - [x] price_history 表 - 支持OHLC数据和技术分析
    - [x] news_events 表 - 新闻事件数据存储
    - [x] technical_signals 表 - 技术指标信号
    - [x] market_anomalies 表 - 市场异动记录
  - [x] SQLite语法兼容性优化
  - [x] 数据库迁移工具自动化

#### 13. 开发工具和项目结构优化 (2025-06-14)
- [x] 项目文件结构重新整理
  - [x] 文档分类：development/ deployment/ guides/
  - [x] 脚本分类：development/ deployment/ testing/
  - [x] 根目录整洁化
- [x] 开发工具脚本完善
  - [x] scripts/development/new_migration.ps1 - 一键创建迁移文件
  - [x] scripts/development/dev_migrate.ps1 - 运行数据库迁移
  - [x] scripts/development/dev_start.ps1 - 开发环境一键启动
- [x] AI协作优化
  - [x] 创建 AI_CONTEXT.md - AI助手快速上手指南
  - [x] 完善项目结构文档
  - [x] 数据库迁移指南更新

### 🚀 下一阶段开发计划

#### 第一优先级：多市场支持和移动端优化 ⭐ *当前焦点*
- [x] 需求分析和设计
  - [x] 多市场支持需求文档 ([详见需求文档](../Requirement/MULTI_MARKET_REQUIREMENTS.md))
  - [x] 移动端设计需求文档 ([详见设计文档](../Requirement/MOBILE_DESIGN_REQUIREMENTS.md))
  - [x] 需求管理流程建立 ([详见管理总览](../Requirement/README.md))
- [ ] 用户反馈迭代
  - [x] 界面易用性评估
    - [x] 修复移动端中文输入法重复字符问题
  - [ ] 功能完整性验证  
  - [ ] 性能和稳定性测试
  - [ ] 新功能需求收集

#### 第二优先级：产品化增强
- [ ] 代码质量提升
  - [ ] 清理编译警告
  - [ ] 添加单元测试覆盖
  - [ ] 错误处理完善
  - [ ] 日志系统优化
  - [ ] Docker镜像优化

- [ ] 用户体验细节
  - [x] 股票代码智能提示（已完成）
  - [ ] 价格趋势简单图表
  - [ ] 操作成功/失败Toast提示
  - [ ] Docker部署文档

#### 第三优先级：市场异动监控系统
- [x] 数据库基础架构 (2025-06-14完成)
  - [x] price_history 表设计和实现
  - [x] market_anomalies 表设计和实现
  - [x] technical_signals 表设计和实现
  - [x] news_events 表设计和实现
- [ ] 价格异动检测算法实现
  - [ ] 暴涨暴跌检测（±5%）
  - [ ] 成交量异常检测（3倍平均量）
  - [ ] 技术位突破检测
  - [ ] 盘前盘后异动监控

- [ ] 技术指标系统
  - [ ] 移动平均穿越检测
  - [ ] RSI极值信号
  - [ ] MACD金叉死叉
  - [ ] 信号评级机制

- [ ] 新闻事件监控
  - [ ] 个股重大新闻监控
  - [ ] 行业政策变化监控
  - [ ] 宏观经济事件监控
  - [ ] 简单情感分析

#### 第四优先级：智能过滤和个性化
- [ ] 噪音过滤机制
  - [ ] 重复提醒过滤
  - [ ] 市场环境过滤
  - [ ] 用户偏好配置

- [ ] 提醒优先级系统
  - [ ] 多级优先级分类
  - [ ] 智能提醒调度
  - [ ] 用户自定义规则

#### 第五优先级：高级功能
- [ ] 事件日历
  - [ ] Fed会议日程
  - [ ] 财报发布日期
  - [ ] 经济数据发布
  - [ ] 期权到期日

- [ ] 市场情绪指标
  - [ ] VIX恐慌指数监控
  - [ ] 资金流向分析
  - [ ] 板块轮动信号

## 技术实现方案

### 数据库扩展计划
```sql
-- 价格历史表（✅ 2025-06-14 已实现）
CREATE TABLE price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    date DATE NOT NULL,
    open_price REAL NOT NULL,
    high_price REAL NOT NULL,
    low_price REAL NOT NULL,
    close_price REAL NOT NULL,
    volume INTEGER NOT NULL,
    daily_change_percent REAL,
    volume_ratio REAL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 新闻事件表（✅ 2025-06-14 已实现）
CREATE TABLE news_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    source TEXT NOT NULL,
    sentiment TEXT,
    event_type TEXT NOT NULL,
    published_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 技术指标表（✅ 2025-06-14 已实现）
CREATE TABLE technical_signals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    indicator_type TEXT NOT NULL,
    signal_value REAL NOT NULL,
    signal_strength INTEGER,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 异动记录表（✅ 2025-06-14 已实现）
CREATE TABLE market_anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    anomaly_type TEXT NOT NULL,
    current_price REAL NOT NULL,
    change_percent REAL NOT NULL,
    volume_ratio REAL NOT NULL,
    description TEXT,
    severity INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 相关索引（✅ 2025-06-14 已实现）
CREATE INDEX idx_price_history_symbol_date ON price_history(symbol, date);
CREATE INDEX idx_news_events_symbol_published ON news_events(symbol, published_at);
CREATE INDEX idx_technical_signals_symbol_type ON technical_signals(symbol, indicator_type);
CREATE INDEX idx_market_anomalies_symbol_type ON market_anomalies(symbol, anomaly_type);
```

## 开发环境和工具

### 核心依赖
```toml
axum = "0.6"           # Web框架
sqlx = "0.7"           # 数据库ORM
tokio = "1.0"          # 异步运行时
serde = "1.0"          # 序列化
lettre = "0.11"        # 邮件发送
askama = "0.12"        # 模板引擎
```

### 开发工具
- `cargo run --bin trade_alert_rust` - 主程序
- `cargo run --bin test_email` - 邮件测试
- `cargo run --bin test_network` - 网络诊断
- `test_api.ps1` - API功能测试
- `test_email.ps1` - 邮件配置测试

## 性能指标

### 当前性能
- 启动时间：< 1秒
- API响应时间：< 100ms
- 价格检查间隔：30秒
- 邮件发送时间：< 5秒

### 目标性能
- 支持并发用户：100+
- 价格数据延迟：< 1分钟
- 预警触发延迟：< 30秒
- 系统可用性：99.9%
- 异动提醒及时性：5分钟内
- 提醒准确率：80%以上

## 成功指标

### 用户价值验证
- 每周能捕捉到2-3次有价值的市场异动
- 提醒及时性：异动发生后5分钟内通知
- 准确率：80%以上的提醒确实值得关注
- 用户满意度：持续使用并推荐给朋友

### 技术性能指标
- 数据延迟：<1分钟
- 系统可用性：99%+
- 提醒发送成功率：95%+
- API响应时间：<200ms

---

**最后更新**: 2025-06-14 15:00
**更新人**: AI Assistant  
**下次review**: 基于市场异动监控算法开发进度
**当前焦点**: 市场异动监控系统开发 > 产品化增强

## 2025-06-14 重要进展总结

### 🎯 今日完成的关键任务
1. **数据库迁移系统完善** - 修复了应用代码与数据库表结构不匹配的问题
2. **市场异动监控基础架构** - 完成了所有相关数据表的设计和实现
3. **项目结构优化** - 重新整理了文档和脚本的目录结构
4. **开发工具完善** - 创建了自动化的数据库迁移和开发启动脚本
5. **AI协作优化** - 建立了AI助手快速上手的标准化流程

### 🚀 下一步重点
- 开始实现价格异动检测算法
- 基于新的数据表结构开发技术指标系统
- 完善市场异动监控的核心逻辑 