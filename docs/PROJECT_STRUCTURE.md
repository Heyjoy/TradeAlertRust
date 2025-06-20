# 项目结构说明

## 📁 目录结构

```
TradeAlertRust/
├── 📂 src/                     # 源代码
│   ├── main.rs                 # 主程序入口
│   ├── db.rs                   # 数据库模块
│   ├── email.rs                # 邮件通知模块
│   ├── fetcher.rs              # 价格获取模块
│   ├── models.rs               # 数据模型
│   ├── config.rs               # 配置管理
│   └── templates.rs            # 模板渲染
│
├── 📂 docs/                    # 文档目录
│   ├── 📂 Requirement/         # 需求管理文档 ⭐ *v2.0*
│   │   ├── README.md                    # 需求管理总览
│   │   ├── PRD_MASTER.md                # 主PRD文档 (v2.0)
│   │   ├── REQUIREMENT_TRACEABILITY.md  # 需求追溯性管理
│   │   ├── MULTI_MARKET_REQUIREMENTS.md # 多市场支持需求
│   │   ├── MOBILE_DESIGN_REQUIREMENTS.md # 移动端设计需求
│   │   └── 尾盘牵牛-战法说明.pptx       # 交易策略文档
│   │
│   ├── 📂 archive/             # 归档文档 📁 *新增*
│   │   ├── README.md                    # 归档说明
│   │   └── PRD_v1.0.md                  # v1.0产品需求文档 (已归档)
│   │
│   ├── 📂 development/         # 开发相关文档
│   │   ├── DATABASE_MIGRATION_GUIDE.md  # 数据库迁移指南
│   │   ├── DEVELOPMENT_PLAN.md          # 开发计划
│   │   └── user_feedback.md             # 用户反馈记录
│   │
│   ├── 📂 deployment/          # 部署相关文档
│   │   ├── QUICK_DEPLOY.md              # 快速部署指南
│   │   ├── RAILWAY_DEPLOY_GUIDE.md      # Railway部署指南
│   │   ├── RAILWAY_SECURITY_GUIDE.md    # Railway安全指南
│   │   └── security-config.md           # 安全配置文档
│   │
│   ├── 📂 guides/              # 使用指南
│   │   ├── QUICK_EMAIL_SETUP.md         # 邮件设置指南
│   │   ├── USEFUL_SCRIPTS.md            # 实用脚本说明
│   │   ├── friend_test_guide.md         # 朋友测试指南
│   │   └── setup_example.md             # 设置示例
│   │
│   └── PROJECT_STRUCTURE.md    # 项目结构说明（本文件）
│
├── 📂 scripts/                 # 脚本目录
│   ├── 📂 development/         # 开发脚本
│   │   ├── dev_start.ps1                # 开发环境启动
│   │   ├── dev_migrate.ps1              # 数据库迁移
│   │   ├── new_migration.ps1            # 创建新迁移
│   │   ├── start.ps1                    # 简单启动脚本
│   │   ├── start.bat                    # 批处理启动脚本
│   │   └── run_server.bat               # 服务器运行脚本
│   │
│   ├── 📂 deployment/          # 部署脚本
│   │   ├── cross_compile_arm.ps1        # ARM交叉编译
│   │   ├── deploy_nas.sh                # NAS部署
│   │   ├── deploy_nas_direct.sh         # NAS直接部署
│   │   ├── setup_ddns.sh                # DDNS设置
│   │   ├── deploy_to_railway.ps1        # Railway部署
│   │   ├── start_public.ps1             # 公网启动脚本
│   │   └── start_public.bat             # 公网启动批处理
│   │
│   └── 📂 testing/             # 测试脚本
│       ├── test_email.ps1               # 邮件测试
│       ├── test_yahoo_api.ps1           # Yahoo API测试
│       └── test_api.ps1                 # API测试
│
├── 📂 migrations/              # 数据库迁移文件
├── 📂 templates/               # HTML模板
├── 📂 data/                    # 数据文件（SQLite数据库）
├── 📂 docker/                  # Docker相关文件
├── 📂 nginx/                   # Nginx配置
├── 📂 synology/                # 群晖NAS相关
│
├── 📄 README.md                # 项目说明
├── 📄 Cargo.toml               # Rust项目配置
├── 📄 config.toml              # 应用配置
├── 📄 config.toml.example      # 配置示例
├── 📄 _env.example             # 环境变量示例
└── 📄 railway.env.example      # Railway环境变量示例
```

## 🎯 文件分类说明

### 📚 文档分类 (`docs/`)

| 目录 | 用途 | 包含文件 |
|------|------|----------|
| `Requirement/` | 需求管理文档 | PRD主文档、需求追溯管理、技术设计需求 |
| `archive/` | 归档文档 | 已过时或被替代的历史文档 |
| `development/` | 开发相关文档 | 数据库迁移指南、开发计划、用户反馈 |
| `deployment/` | 部署相关文档 | 部署指南、安全配置、Railway相关 |
| `guides/` | 使用指南 | 邮件设置、脚本说明、测试指南 |

### 🛠️ 脚本分类 (`scripts/`)

| 目录 | 用途 | 包含文件 |
|------|------|----------|
| `development/` | 开发环境脚本 | 启动、迁移、本地开发相关 |
| `deployment/` | 部署脚本 | 交叉编译、NAS部署、Railway部署 |
| `testing/` | 测试脚本 | API测试、邮件测试、功能测试 |

## 🚀 常用操作

### 开发环境
```powershell
# 创建新迁移
.\scripts\development\new_migration.ps1 "add_feature"

# 运行迁移
.\scripts\development\dev_migrate.ps1

# 启动开发环境
.\scripts\development\dev_start.ps1
```

### 测试
```powershell
# 测试邮件功能
.\scripts\testing\test_email.ps1

# 测试API
.\scripts\testing\test_api.ps1

# 测试Yahoo API
.\scripts\testing\test_yahoo_api.ps1
```

### 部署
```powershell
# Railway部署
.\scripts\deployment\deploy_to_railway.ps1

# NAS部署
.\scripts\deployment\deploy_nas.sh

# ARM交叉编译
.\scripts\deployment\cross_compile_arm.ps1
```

## 📋 文档索引

### 📋 需求管理
- [需求管理总览](Requirement/README.md) - 需求管理流程和文档组织
- [主PRD文档](Requirement/PRD_MASTER.md) - 产品需求主文档 (v2.0)
- [需求追溯性管理](Requirement/REQUIREMENT_TRACEABILITY.md) - 需求一致性和追溯性
- [多市场支持需求](Requirement/MULTI_MARKET_REQUIREMENTS.md) - 美股、A股、加密货币技术需求
- [移动端设计需求](Requirement/MOBILE_DESIGN_REQUIREMENTS.md) - 移动优先的界面设计需求

### 📁 归档文档
- [归档说明](archive/README.md) - 归档文档管理说明
- [v1.0 PRD文档](archive/PRD_v1.0.md) - 历史版本产品需求文档

### 🔧 开发相关
- [数据库迁移指南](development/DATABASE_MIGRATION_GUIDE.md) - 数据库迁移的完整流程
- [开发计划](development/DEVELOPMENT_PLAN.md) - 项目开发路线图
- [用户反馈](development/user_feedback.md) - 用户反馈收集和处理

### 🚀 部署相关
- [快速部署](deployment/QUICK_DEPLOY.md) - 快速部署到各种环境
- [Railway部署](deployment/RAILWAY_DEPLOY_GUIDE.md) - Railway平台部署详细指南
- [安全配置](deployment/security-config.md) - 生产环境安全配置

### 📖 使用指南
- [邮件设置](guides/QUICK_EMAIL_SETUP.md) - 邮件通知功能设置
- [产品文档](guides/PRD.md) - 产品需求和功能说明
- [实用脚本](guides/USEFUL_SCRIPTS.md) - 各种实用脚本的使用方法

## 🎉 整理后的优势

1. **📁 结构清晰** - 文档和脚本按功能分类，易于查找
2. **🔍 快速定位** - 根据需求直接找到对应目录
3. **🛠️ 开发效率** - 开发、测试、部署脚本分离，避免混淆
4. **📚 文档管理** - 不同类型文档分类存放，便于维护
5. **🚀 新手友好** - 清晰的目录结构降低学习成本

现在你可以根据需要快速找到相应的文档和脚本！ 