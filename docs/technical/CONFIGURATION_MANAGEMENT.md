# TradeAlert 配置管理规则

## 🚀 AI快速启动 (新对话必读)

### 环境变量命名规则 (最重要)
**格式**: `TRADE_ALERT__<SECTION>__<KEY>`
**分隔符**: 双下划线 `__` (避免字段名冲突)

**核心示例**:
```bash
TRADE_ALERT__EMAIL__SMTP_USERNAME=your_email@gmail.com
TRADE_ALERT__EMAIL__SMTP_PASSWORD=your_app_password
TRADE_ALERT__SERVER__PORT=8000
TRADE_ALERT__LOGGING__LEVEL=info
```

### 快速导航 (文件位置)
- **源代码**: `src/` - 所有Rust模块
- **文档**: `docs/` - 按功能分类的文档目录
- **脚本**: `scripts/` - 按用途分类的脚本目录
- **配置**: 根目录的配置文件和环境变量文件

### 安全规则 (绝对重要)
**绝对不要提交到Git**:
- 邮箱密码
- API密钥
- 包含密码的数据库连接字符串

**推荐配置方式**:
1. **环境变量** (最高优先级)
2. **Railway Sealed Variables** (生产环境)
3. **本地配置文件** (.gitignore排除)

### 配置层级 (优先级从高到低)
1. **环境变量** - `TRADE_ALERT__<SECTION>__<KEY>`
2. **主配置文件** - `config.toml` (被.gitignore排除)
3. **本地配置文件** - `config.local.toml` (被.gitignore排除)
4. **配置模板** - `config.toml.example` (公开模板)

## 📝 文档命名规则

### 文档层级编号
- **0.x**: 管理策略层 - 项目管理策略、流程规范、标准定义、配置管理
- **1.x**: 产品管理层 - 战略规划、总体需求、管理流程、产品文档
- **2.x**: 功能需求层 - 具体功能模块、用户需求、业务流程
- **3.x**: 实现规格层 - 详细技术规格、算法设计、接口定义、具体实现

### 文件命名规范
**格式**: `层级.序号-功能描述.md`
**示例**: 
- `0.0-REQUIREMENT_MANAGEMENT_STRATEGY.md`
- `0.1-CONFIGURATION_MANAGEMENT.md`
- `1.0-CORE_FEATURES.md`
- `1.1-PRD_MASTER.md`
- `2.1-MULTI_MARKET.md`
- `3.1-LIMIT_UP_PULLBACK.md`

### 特殊文档命名规则

#### 管理类文档 (Management Documents)
- **格式**: `管理类型_具体名称.md`
- **示例**: 
  - `PRIORITY_MATRIX.md`
  - `DECISION_LOG.md`
  - `REQUIREMENT_KANBAN.md`
  - `CURRENT_STATUS.md`

#### 报告类文档 (Report Documents)
- **格式**: `报告类型_时间范围_内容.md`
- **示例**:
  - `PHASE1_COMPLETION_REPORT.md`
  - `PHASE2_IMPROVEMENT_SUMMARY.md`

#### 技术分析文档 (Technical Analysis)
- **格式**: `技术领域_具体分析.md`
- **示例**:
  - `A_SHARE_DATA_SOURCE_ANALYSIS.md`
  - `REQUIREMENT_TRACEABILITY_ANALYSIS.md`

### 需求编号规范
- **FR-xxx**: 功能需求 (Functional Requirements)
- **TR-xxx**: 技术需求 (Technical Requirements)
- **NFR-P-xxx**: 性能需求 (Performance)
- **NFR-U-xxx**: 可用性需求 (Usability)
- **NFR-R-xxx**: 可靠性需求 (Reliability)
- **CR-xxx**: 约束需求 (Constraint Requirements)

## 📁 文件夹结构和用途

### 单一数据源原则
**重要**: 遵循"单一数据源"(Single Source of Truth)原则，避免重复维护相同信息。

- **项目结构**: 详见 [项目结构说明](../PROJECT_STRUCTURE.md)
- **文档组织**: 详见 [需求管理总览](../Requirement/README.md)
- **脚本分类**: 详见 [实用脚本说明](../guides/USEFUL_SCRIPTS.md)

## 📁 配置文件位置

### 核心配置文件
- `config.toml.example` - 配置模板 (可提交到git)
- `config.local.toml` - 本地配置 (被.gitignore排除)
- `config.toml` - 主配置 (被.gitignore排除)
- `.env` - 环境变量文件 (被.gitignore排除)

### 环境变量示例文件
- `_env.example` - 本地开发环境变量模板
- `railway.env.example` - Railway部署环境变量模板

### 配置相关文档
- `docs/deployment/security-config.md` - 安全配置指南
- `docs/deployment/RAILWAY_DEPLOY_GUIDE.md` - Railway部署指南
- `docs/guides/setup_example.md` - 快速设置示例

## 🔧 配置加载机制

### 代码位置
- **配置结构定义**: `src/config.rs`
- **配置加载逻辑**: `Config::load()` 方法
- **环境变量处理**: `handle_production_env()` 和 `resolve_placeholders()`

### 特殊处理
- **Railway适配**: 自动处理PORT环境变量和数据库路径
- **占位符解析**: 支持 `${VAR_NAME}` 语法
- **生产环境**: 自动使用 `data/` 目录

## 🚀 快速配置

### 本地开发
```bash
# 方法1: 使用.env文件
cp _env.example .env
# 编辑.env文件填入真实值

# 方法2: 使用本地配置文件
cp config.toml.example config.local.toml
# 编辑config.local.toml填入真实值
```

### Railway部署
- 在Railway Dashboard的Variables页面添加环境变量
- 参考 `railway.env.example` 中的变量列表
- 使用Sealed Variables保护敏感信息

## 📊 验证配置

### 测试命令
```bash
# 测试邮件配置
curl http://localhost:3000/api/test-email

# 查看配置加载
RUST_LOG=debug cargo run
```

---

**相关文档**: 
- [安全配置指南](../deployment/security-config.md) - 详细安全配置
- [Railway部署指南](../deployment/RAILWAY_DEPLOY_GUIDE.md) - 生产环境部署
- [快速设置示例](../guides/setup_example.md) - 新手配置步骤
- [需求管理策略](../Requirement/0.0-REQUIREMENT_MANAGEMENT_STRATEGY.md) - 文档命名详细规范
- [项目结构说明](../PROJECT_STRUCTURE.md) - 完整的目录结构说明 