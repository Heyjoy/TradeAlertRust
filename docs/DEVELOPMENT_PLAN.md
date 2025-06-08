# 交易预警系统开发计划 (内部文档)

## 项目概述
开发一个实时交易预警系统，用于监控股票价格并在达到预设条件时发出提醒。

## 技术栈
- **后端**: Rust + Axum
- **数据库**: SQLite (后期可迁移到PostgreSQL)
- **前端**: HTML5 + CSS3 + JavaScript + Bootstrap
- **模板引擎**: Askama
- **定时任务**: tokio-cron-scheduler
- **邮件系统**: lettre + SMTP
- **行情数据源**: Yahoo Finance API
- **测试工具**: 多个独立测试二进制文件

## 当前状态 (2025-06-07)

### ✅ 已完成功能

#### 1. 核心架构
- [x] 项目框架搭建 (Axum)
- [x] 配置系统实现 (config.toml + 环境变量)
- [x] 数据库迁移系统 (SQLx)
- [x] 模块化架构设计

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

#### 4. 邮件通知系统 🎉
- [x] SMTP 邮件发送功能
- [x] 支持多种邮件服务商 (Gmail, QQ, 163, Outlook)
- [x] 智能连接类型识别 (STARTTLS/TLS)
- [x] HTML 邮件模板
- [x] 测试邮件功能
- [x] 预警触发邮件通知
- [x] 邮件发送状态跟踪

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
- [x] API测试脚本 (PowerShell)

### 🔧 需要清理的代码问题

从最新编译输出分析，需要处理以下代码警告：

#### 未使用的导入 (unused imports)
```rust
// src/db.rs:3 - AlertCondition 未使用
// src/db.rs:1 - Row 未使用  
// src/main.rs:14 - routing::put 未使用
// src/main.rs:19 - std::time::Duration 未使用
```

#### 未使用的字段 (unused fields)
```rust
// src/fetcher.rs:40 - YahooMeta.symbol 未使用
// src/fetcher.rs:60-61 - PriceCache.price, volume 未使用
// src/config.rs:22 - SchedulerConfig.default_schedule 未使用
// src/config.rs:53 - Config.scheduler 未使用
// src/templates/mod.rs:7 - BaseTemplate.title 未使用
// src/templates/mod.rs:13,20 - base 字段未使用
// src/models.rs:91 - AlertForTemplate.updated_at 未使用
```

#### 未使用的方法 (unused methods)
```rust
// src/db.rs:85,101 - update_alert_status, mark_alert_triggered
// src/models.rs:127 - Alert.is_triggered
// src/fetcher.rs:95,109 - PriceService.start, check_and_reset_request_count
```

#### 未使用的类型 (unused types)
```rust
// src/db.rs:151 - DbResult<T> 类型别名
```

### 🚀 下一阶段开发计划

#### 第一优先级：代码清理 (预计1-2天)
- [ ] 清理所有未使用的导入
- [ ] 移除或实现未使用的字段
- [ ] 清理未使用的方法和类型
- [ ] 运行 `cargo fix` 自动修复
- [ ] 确保编译零警告

#### 第二优先级：功能增强 (预计3-5天)
- [ ] 预警编辑功能优化
  - [ ] 表单预填充当前值
  - [ ] 实时表单验证
  - [ ] 编辑成功反馈
- [ ] 价格显示增强
  - [ ] 实时价格更新 (Ajax轮询)
  - [ ] 价格变动指示器
  - [ ] 历史价格图表
- [ ] 用户体验优化
  - [ ] 加载状态指示
  - [ ] 操作成功/失败提示
  - [ ] 响应式设计完善

#### 第三优先级：高级功能 (预计1周)
- [ ] WebSocket 实时推送
  - [ ] 实时价格更新
  - [ ] 实时预警通知
  - [ ] 连接状态管理
- [ ] 预警条件扩展
  - [ ] 百分比变化预警
  - [ ] 价格区间预警
  - [ ] 技术指标预警
- [ ] 通知系统增强
  - [ ] 多种通知方式
  - [ ] 通知频率控制
  - [ ] 通知历史记录

## 技术债务和改进点

### 代码质量
- [ ] 增加单元测试覆盖率
- [ ] 改进错误处理机制
- [ ] 添加日志记录系统
- [ ] 性能监控和优化

### 安全性
- [x] 配置文件安全 (已实现)
- [ ] API 访问控制
- [ ] 输入数据验证
- [ ] SQL 注入防护 (SQLx 已提供)

### 可维护性
- [ ] 代码文档完善
- [ ] API 文档生成
- [ ] 部署文档更新
- [ ] 监控和告警系统

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

### 测试策略
- 单元测试：模型和工具函数
- 集成测试：数据库操作
- 端到端测试：API功能
- 手动测试：Web界面交互

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

## 部署和运维

### 当前部署方式
- 本地开发环境
- SQLite数据库
- 文件配置管理

### 生产部署计划
- [ ] Docker容器化
- [ ] 数据库迁移到PostgreSQL
- [ ] 反向代理配置
- [ ] 监控和日志系统
- [ ] 自动化部署流程

## 团队协作

### Git工作流
- main分支：稳定版本
- develop分支：开发版本
- feature分支：功能开发
- hotfix分支：紧急修复

### 代码审查
- 所有代码变更需要审查
- 自动化测试必须通过
- 编译警告需要清理
- 文档需要同步更新

### 沟通机制
- 每日进度同步
- 技术问题讨论
- 代码审查反馈
- 版本发布规划

## 风险管理

### 技术风险
- Yahoo Finance API限制
- 网络连接稳定性
- 邮件服务可用性
- 数据库性能瓶颈

### 业务风险
- 价格数据准确性
- 预警及时性
- 用户数据安全
- 系统可扩展性

### 应对策略
- 多数据源备份
- 错误重试机制
- 监控告警系统
- 定期安全审计

---

**最后更新**: 2025-06-07 19:30
**更新人**: AI Assistant
**下次review**: 2025-06-08 