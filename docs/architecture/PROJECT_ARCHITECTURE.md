# 🏗️ TradeAlertRust 项目架构文档

> 📋 **文档版本**: v2.0  
> 📅 **更新日期**: 2025-01-09  
> 🎯 **涵盖范围**: 完整系统架构 + AI协作基础设施  

## 📖 **架构概览**

TradeAlertRust是一个基于Rust构建的智能交易预警系统，采用现代化的微服务架构设计，集成了完整的AI协作开发基础设施。系统支持多市场（美股、A股、加密货币）的实时价格监控和智能预警功能。

### **🎯 核心特性**
- **多市场支持**: 美股(Yahoo Finance) + A股(新浪/腾讯财经)
- **实时监控**: 30秒级价格更新和预警检查
- **智能通知**: 邮件预警 + 多渠道通知
- **AI协作**: 完整的AI辅助开发工作流程
- **云原生**: 支持Railway、Docker、群晖NAS部署

---

## 🏛️ **系统整体架构**

### **分层架构设计**

```mermaid
graph TB
    %% 用户界面层
    subgraph "🌐 用户界面层"
        WEB[Web界面<br/>HTML/CSS/JS]
        API[REST API<br/>JSON接口]
        CLI[命令行工具<br/>管理脚本]
    end

    %% 应用服务层
    subgraph "⚡ 应用服务层"
        MAIN[主应用<br/>main.rs]
        ROUTER[路由层<br/>Axum Router]
        HANDLERS[处理器<br/>API Handlers]
        MIDDLEWARE[中间件<br/>日志/追踪]
    end

    %% 业务逻辑层
    subgraph "🧠 业务逻辑层"
        ALERT[预警引擎<br/>Alert Logic]
        PRICE[价格服务<br/>Price Service]
        EMAIL[邮件服务<br/>Email Service]
        SCHEDULER[任务调度<br/>Cron Jobs]
    end

    %% 数据访问层
    subgraph "💾 数据访问层"
        DB[数据库层<br/>SQLx + SQLite]
        CACHE[缓存层<br/>内存缓存]
        MODELS[数据模型<br/>Rust Structs]
    end

    %% 外部服务层
    subgraph "🌍 外部服务层"
        YAHOO[Yahoo Finance<br/>美股数据]
        SINA[新浪财经<br/>A股数据]
        TENCENT[腾讯财经<br/>A股数据]
        SMTP[SMTP服务<br/>邮件发送]
    end

    %% 配置和工具层
    subgraph "🔧 配置和工具层"
        CONFIG[配置管理<br/>TOML/ENV]
        LOGGING[日志系统<br/>Tracing]
        MIGRATE[数据库迁移<br/>SQLx Migrate]
        TEMPLATES[模板引擎<br/>Askama]
    end

    %% AI协作基础设施
    subgraph "🤖 AI协作基础设施"
        CURSOR[Cursor AI<br/>开发环境]
        RULES[开发规范<br/>.cursorrules]
        MODES[专家模式<br/>modes.json]
        WORKFLOW[协作流程<br/>Scripts]
    end

    %% 部署和运维
    subgraph "🚀 部署和运维"
        RAILWAY[Railway平台<br/>云部署]
        DOCKER[Docker容器<br/>容器化]
        SYNOLOGY[群晖NAS<br/>私有部署]
        MONITOR[监控告警<br/>系统监控]
    end

    %% 文档和管理
    subgraph "📚 文档和管理"
        DOCS[项目文档<br/>Markdown]
        TASKS[任务管理<br/>Tasks Tracking]
        STATUS[状态跟踪<br/>Development Status]
        GUIDES[操作指南<br/>User Guides]
    end

    %% 连接关系
    WEB --> ROUTER
    API --> ROUTER
    CLI --> MAIN
    
    ROUTER --> HANDLERS
    HANDLERS --> ALERT
    HANDLERS --> PRICE
    HANDLERS --> EMAIL
    
    ALERT --> DB
    PRICE --> DB
    PRICE --> CACHE
    EMAIL --> SMTP
    
    SCHEDULER --> PRICE
    SCHEDULER --> ALERT
    
    PRICE --> YAHOO
    PRICE --> SINA
    PRICE --> TENCENT
    
    DB --> MODELS
    MAIN --> CONFIG
    MAIN --> LOGGING
    HANDLERS --> TEMPLATES
    
    CURSOR --> RULES
    CURSOR --> MODES
    CURSOR --> WORKFLOW
    
    MAIN --> RAILWAY
    RAILWAY --> DOCKER
    DOCKER --> SYNOLOGY
    
    DOCS --> TASKS
    DOCS --> STATUS
    DOCS --> GUIDES
```

---

## 🔄 **数据流架构**

### **请求处理和数据流向**

```mermaid
graph LR
    %% 数据流架构图
    subgraph "📱 前端界面"
        UI1[Web表单<br/>创建预警]
        UI2[预警列表<br/>管理界面]
        UI3[价格图表<br/>历史数据]
    end

    subgraph "🔄 API路由层"
        R1[POST /api/alerts<br/>创建预警]
        R2[GET /api/alerts<br/>获取列表]
        R3[GET /api/prices/:symbol<br/>价格查询]
        R4[DELETE /api/alerts/:id<br/>删除预警]
    end

    subgraph "⚙️ 业务处理层"
        H1[创建预警处理器<br/>validate + store]
        H2[预警查询处理器<br/>fetch + format]
        H3[价格查询处理器<br/>cache + fetch]
        H4[预警删除处理器<br/>auth + delete]
    end

    subgraph "💡 核心服务"
        S1[预警引擎<br/>条件匹配逻辑]
        S2[价格服务<br/>数据获取+缓存]
        S3[邮件服务<br/>通知发送]
        S4[调度服务<br/>定时任务]
    end

    subgraph "💾 数据存储"
        DB1[(alerts表<br/>预警配置)]
        DB2[(price_history表<br/>价格历史)]
        CACHE1[内存缓存<br/>实时价格]
    end

    subgraph "🌐 外部API"
        EXT1[Yahoo Finance<br/>美股实时价格]
        EXT2[新浪财经<br/>A股实时价格]
        EXT3[腾讯财经<br/>A股备用数据]
        EXT4[SMTP服务器<br/>邮件发送]
    end

    subgraph "⏰ 后台任务"
        T1[价格更新任务<br/>每30秒执行]
        T2[预警检查任务<br/>价格变化触发]
        T3[缓存清理任务<br/>定期清理]
        T4[邮件发送任务<br/>异步队列]
    end

    %% 数据流连接
    UI1 --> R1
    UI2 --> R2
    UI3 --> R3
    
    R1 --> H1
    R2 --> H2
    R3 --> H3
    R4 --> H4
    
    H1 --> S1
    H2 --> S1
    H3 --> S2
    H4 --> S1
    
    S1 --> DB1
    S2 --> DB2
    S2 --> CACHE1
    S3 --> EXT4
    
    S2 --> EXT1
    S2 --> EXT2
    S2 --> EXT3
    
    S4 --> T1
    T1 --> S2
    T2 --> S1
    T2 --> S3
    T3 --> CACHE1
    T4 --> S3
```

---

## 🤖 **AI协作基础设施架构**

### **完整的AI辅助开发工作流程**

```mermaid
graph TD
    %% AI协作工作流程架构
    subgraph "🚀 工作会话启动"
        START[开始工作会话]
        ENV_CHECK[环境检查脚本<br/>check_env.bat]
        GIT_STATUS[Git状态检查<br/>未提交更改]
        COMPILE[项目编译验证<br/>cargo check]
        CONTEXT[AI上下文加载<br/>核心文档读取]
    end

    subgraph "🎭 AI专家模式系统"
        MODE_SELECT[专家模式选择]
        RE[Rust Expert<br/>核心开发]
        TA[Trading Analyst<br/>交易逻辑]
        SA[Security Auditor<br/>安全审查]
        DO[DevOps Engineer<br/>部署运维]
        AR[System Architect<br/>架构设计]
        DW[Documentation Writer<br/>文档编写]
        QA[QA Engineer<br/>测试质量]
    end

    subgraph "📋 规范和规则系统"
        CURSORRULES[.cursorrules<br/>主规范文件]
        RUST_RULES[rust-rules.mdc<br/>Rust开发规范]
        TRADING_RULES[trading-rules.mdc<br/>交易业务规则]
        SECURITY_RULES[security-rules.mdc<br/>安全开发规范]
        MODES_CONFIG[modes.json<br/>专家模式配置]
    end

    subgraph "🔄 标准开发循环"
        ANALYZE[任务分析阶段<br/>需求+方案+风险]
        IMPLEMENT[代码实现阶段<br/>编码+审查+测试]
        VERIFY[验证确认阶段<br/>质量+功能+文档]
        ITERATE[迭代改进<br/>反馈+优化]
    end

    subgraph "📊 工作会话收尾"
        QUALITY[代码质量检查<br/>clippy+test+fmt]
        DOCS_UPDATE[文档更新<br/>任务状态+进展]
        GIT_COMMIT[版本控制<br/>提交+推送]
        SUMMARY[工作总结<br/>成果+计划]
    end

    %% 工作流程连接
    START --> ENV_CHECK
    ENV_CHECK --> GIT_STATUS
    GIT_STATUS --> COMPILE
    COMPILE --> CONTEXT
    CONTEXT --> MODE_SELECT
    
    MODE_SELECT --> RE
    MODE_SELECT --> TA
    MODE_SELECT --> SA
    MODE_SELECT --> DO
    MODE_SELECT --> AR
    MODE_SELECT --> DW
    MODE_SELECT --> QA
    
    RE --> ANALYZE
    TA --> ANALYZE
    SA --> ANALYZE
    DO --> ANALYZE
    AR --> ANALYZE
    DW --> ANALYZE
    QA --> ANALYZE
    
    ANALYZE --> IMPLEMENT
    IMPLEMENT --> VERIFY
    VERIFY --> ITERATE
    ITERATE --> ANALYZE
    
    VERIFY --> QUALITY
    QUALITY --> DOCS_UPDATE
    DOCS_UPDATE --> GIT_COMMIT
    GIT_COMMIT --> SUMMARY
    
    %% 规范系统连接
    CURSORRULES --> RE
    RUST_RULES --> RE
    TRADING_RULES --> TA
    SECURITY_RULES --> SA
    MODES_CONFIG --> MODE_SELECT
```

---

## 📁 **项目目录结构架构**

### **完整的项目组织结构**

```mermaid
graph TB
    %% 项目目录结构架构
    subgraph "📁 项目根目录 - TradeAlertRust"
        ROOT[TradeAlertRust/]
    end

    subgraph "🦀 Rust核心代码"
        SRC[src/<br/>源代码目录]
        CARGO[Cargo.toml<br/>项目配置]
        
        subgraph "源代码模块"
            MAIN[main.rs<br/>应用入口]
            CONFIG_RS[config.rs<br/>配置管理]
            DB_RS[db.rs<br/>数据库层]
            MODELS_RS[models.rs<br/>数据模型]
            FETCHER_RS[fetcher.rs<br/>价格获取]
            EMAIL_RS[email.rs<br/>邮件服务]
        end
    end

    subgraph "🤖 AI协作基础设施"
        CURSOR_DIR[.cursor/<br/>AI配置目录]
        CURSORRULES[.cursorrules<br/>主规范文件]
        
        subgraph "AI配置文件"
            MODES_JSON[modes.json<br/>专家模式]
            RULES_DIR[rules/<br/>规则目录]
        end
    end

    subgraph "📚 文档系统"
        DOCS[docs/<br/>文档目录]
        
        subgraph "核心文档"
            START_HERE_FILE[START_HERE.md<br/>启动指南]
            WORKFLOW_FILE[ai-collaboration-workflow.md<br/>协作流程]
            ARCHITECTURE_FILE[PROJECT_ARCHITECTURE.md<br/>架构文档]
        end
    end

    subgraph "📋 任务和状态管理"
        TASKS[tasks/<br/>任务目录]
        CURRENT_TASKS_FILE[current-tasks.md<br/>当前任务]
        DEV_STATUS_FILE[development-status.md<br/>开发状态]
    end

    subgraph "🔧 脚本和工具"
        SCRIPTS[scripts/<br/>脚本目录]
        
        subgraph "开发脚本"
            START_SESSION[start_work_session.bat]
            END_SESSION[end_work_session.bat]
            CHECK_ENV[check_env.bat]
        end
    end

    subgraph "🗄️ 数据和配置"
        DATA[data/<br/>数据目录]
        MIGRATIONS[migrations/<br/>数据库迁移]
        CONFIG_TOML[config.toml.example<br/>配置模板]
    end

    subgraph "🚀 部署和容器"
        DOCKER_DIR[docker/<br/>Docker配置]
        SYNOLOGY[synology/<br/>群晖部署]
        RAILWAY_TOML[railway.toml<br/>Railway配置]
    end

    %% 连接关系
    ROOT --> SRC
    ROOT --> CURSOR_DIR
    ROOT --> DOCS
    ROOT --> TASKS
    ROOT --> SCRIPTS
    ROOT --> DATA
    ROOT --> DOCKER_DIR
    
    SRC --> MAIN
    SRC --> CONFIG_RS
    SRC --> DB_RS
    SRC --> MODELS_RS
    SRC --> FETCHER_RS
    SRC --> EMAIL_RS
    
    CURSOR_DIR --> MODES_JSON
    CURSOR_DIR --> RULES_DIR
    
    DOCS --> START_HERE_FILE
    DOCS --> WORKFLOW_FILE
    DOCS --> ARCHITECTURE_FILE
    
    TASKS --> CURRENT_TASKS_FILE
    DOCS --> DEV_STATUS_FILE
    
    SCRIPTS --> START_SESSION
    SCRIPTS --> END_SESSION
    SCRIPTS --> CHECK_ENV
```

---

## 🔧 **技术栈详解**

### **🦀 后端技术栈**
```yaml
核心框架:
  - Rust: 1.85+ (系统编程语言)
  - Axum: 0.7 (Web框架)
  - Tokio: 1.0 (异步运行时)

数据存储:
  - SQLite: 数据库引擎
  - SQLx: 0.8 (数据库ORM)
  - 内存缓存: HashMap + RwLock

外部集成:
  - Reqwest: 0.11 (HTTP客户端)
  - Serde: 1.0 (序列化/反序列化)
  - Lettre: 0.11 (邮件发送)

工具链:
  - Tracing: 日志和追踪
  - Anyhow/Thiserror: 错误处理
  - Chrono: 时间处理
  - Askama: 模板引擎
```

### **🌐 前端技术栈**
```yaml
模板引擎:
  - Askama: Rust模板引擎
  - HTML5: 标准标记语言
  - CSS3: 样式设计
  - JavaScript: 交互逻辑

API接口:
  - REST API: JSON格式
  - HTTP状态码: 标准化响应
  - CORS支持: 跨域访问
```

### **☁️ 部署技术栈**
```yaml
容器化:
  - Docker: 容器技术
  - Docker Compose: 容器编排

云平台:
  - Railway: 主要云部署平台
  - 群晖NAS: 私有化部署选项

监控运维:
  - Tracing: 应用监控
  - 结构化日志: JSON格式
  - 健康检查: HTTP端点
```

---

## 📊 **性能指标和监控**

### **🎯 关键性能指标**
```yaml
响应时间:
  - API响应: < 100ms (95th percentile)
  - 价格更新: 30秒间隔
  - 预警检查: < 5秒

并发处理:
  - 最大并发请求: 100/秒
  - 数据库连接池: 10个连接
  - HTTP客户端池: 复用连接

资源使用:
  - 内存占用: < 100MB
  - CPU使用: < 10% (空闲时)
  - 磁盘空间: < 50MB (数据库)
```

### **📈 监控体系**
```yaml
应用监控:
  - 结构化日志: Tracing框架
  - 错误追踪: 完整堆栈信息
  - 性能指标: 响应时间统计

业务监控:
  - 预警触发率: 成功/失败统计
  - 价格更新频率: 数据源可用性
  - 邮件发送状态: 通知成功率

系统监控:
  - 资源使用: CPU/内存/磁盘
  - 网络连接: 外部API可用性
  - 数据库性能: 查询执行时间
```

---

## 🔒 **安全架构**

### **🛡️ 安全措施**
```yaml
数据保护:
  - 敏感信息: 环境变量存储
  - 数据传输: HTTPS强制加密
  - 数据存储: SQLite文件权限控制

输入验证:
  - API参数: 严格类型检查
  - SQL注入: 参数化查询
  - XSS防护: 模板自动转义

访问控制:
  - API限流: 防止滥用
  - 错误处理: 不泄露内部信息
  - 日志安全: 敏感信息脱敏
```

### **🔐 配置安全**
```yaml
环境变量:
  - 数据库URL: TRADE_ALERT__DATABASE__URL
  - SMTP密码: TRADE_ALERT__EMAIL__SMTP_PASSWORD
  - API密钥: 外部服务认证

文件权限:
  - 配置文件: 只读权限
  - 数据库文件: 应用独占
  - 日志文件: 受限访问
```

---

## 🚀 **部署架构**

### **☁️ 云部署 (Railway)**
```yaml
环境配置:
  - 自动构建: Git推送触发
  - 环境变量: Railway面板配置
  - 域名绑定: 自动HTTPS证书

资源配置:
  - CPU: 共享vCPU
  - 内存: 512MB
  - 存储: 持久化磁盘
  - 网络: 全球CDN
```

### **🏠 私有部署 (群晖NAS)**
```yaml
容器部署:
  - Docker Compose: 服务编排
  - 数据持久化: NAS存储
  - 网络配置: 内网访问

安全配置:
  - 防火墙: 端口访问控制
  - SSL证书: Let's Encrypt
  - 备份策略: 定期数据备份
```

---

## 📋 **开发和维护**

### **🔄 开发工作流程**
```yaml
代码开发:
  1. 环境检查: scripts/development/check_env.bat
  2. AI协作: 选择专家模式
  3. 代码实现: 遵循.cursorrules规范
  4. 质量检查: cargo check/clippy/test/fmt
  5. 文档更新: 同步更新相关文档
  6. 版本控制: Git提交和推送

任务管理:
  - 任务跟踪: tasks/current-tasks.md
  - 进度状态: docs/development-status.md
  - 里程碑: 阶段完成总结
```

### **🧪 测试策略**
```yaml
单元测试:
  - 业务逻辑: 核心算法测试
  - 数据模型: 序列化/反序列化
  - 工具函数: 边界条件测试

集成测试:
  - API端点: 完整请求响应流程
  - 数据库: CRUD操作验证
  - 外部服务: Mock和真实API测试

端到端测试:
  - 用户场景: 完整业务流程
  - 部署验证: 生产环境功能测试
```

---

## 🎯 **未来架构演进**

### **📈 扩展计划**
```yaml
功能扩展:
  - 多策略引擎: 支持复杂交易策略
  - 实时WebSocket: 价格推送优化
  - 移动端API: 原生应用支持
  - 机器学习: 智能预警优化

技术升级:
  - 微服务化: 服务拆分和独立部署
  - 消息队列: 异步任务处理
  - 分布式缓存: Redis集群
  - 负载均衡: 高可用架构

AI协作增强:
  - 智能代码生成: 更高级的AI辅助
  - 自动化测试: AI生成测试用例
  - 性能优化: AI驱动的性能调优
  - 文档智能: 自动文档生成和维护
```

---

## 📞 **相关文档**

### **📚 核心文档链接**
- [项目启动指南](START_HERE.md) - 每次对话必读
- [AI协作工作流程](ai-collaboration-workflow.md) - 详细协作流程
- [开发状态跟踪](development-status.md) - 项目进展状态
- [当前任务管理](../tasks/current-tasks.md) - 任务跟踪
- [配置管理规则](technical/CONFIGURATION_MANAGEMENT.md) - 配置规范

### **🔧 技术文档**
- [数据库迁移指南](DATABASE_MIGRATION_GUIDE.md) - 数据库管理
- [部署指南](deployment/) - 各种部署方式
- [开发指南](development/) - 开发环境设置

---

**📝 文档维护**: 本架构文档随项目发展持续更新  
**🔄 更新频率**: 重大架构变更时更新  
**�� 维护者**: AI助手 + 开发团队 