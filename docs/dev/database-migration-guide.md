# 数据库迁移系统深度解析

## 📋 概述

本文档从系统架构角度深入解释数据库迁移（Database Migration）的原理，以及Rust SQLx独特的编译时SQL检查机制。

## 🏗️ 系统架构：数据库迁移

### 传统开发模式的问题

```
开发者A本地     开发者B本地     测试环境     生产环境
   ↓              ↓             ↓           ↓
手动建表        手动建表       手动建表     手动建表
   ↓              ↓             ↓           ↓
 结构不一致    结构不一致     结构不一致   结构不一致
```

**问题**：
- 团队成员数据库结构不一致
- 生产环境更新容易出错
- 无法回滚到之前的状态
- 不知道当前数据库是什么版本

### 现代迁移系统架构

```
版本控制系统 (Git)
├── migrations/
│   ├── 001_initial.sql
│   ├── 002_add_users.sql  
│   └── 003_add_strategy_signals.sql
│
各环境同步执行迁移
├── 开发环境: sqlx migrate run
├── 测试环境: sqlx migrate run  
├── 生产环境: sqlx migrate run
└── 结果: 所有环境结构一致 ✅
```

**迁移文件 = 数据库的"元数据/蓝图"**

## 🔄 迁移不是语言特有概念

几乎所有现代框架都有迁移系统：
- **Rails**: `rails db:migrate`
- **Django**: `python manage.py migrate`
- **Laravel**: `php artisan migrate`
- **Entity Framework**: `dotnet ef database update`
- **SQLx (Rust)**: `sqlx migrate run`

## 🦀 Rust SQLx的独特创新：编译时SQL检查

### 其他语言：运行时检查

```python
# Python - 运行时才知道错误
cursor.execute("SELECT * FROM strategy_signalss")  # 拼写错误！
# ↑ 只有运行时才会报错：Table doesn't exist
```

```javascript
// JavaScript - 运行时才知道错误  
const result = await db.query("SELECT id, namee FROM users");
// ↑ 拼写错误，运行时才发现
```

### Rust SQLx：编译时检查

```rust
// Rust SQLx - 编译时就检查！
let users = sqlx::query!(
    "SELECT id, namee FROM users"  // 字段名错误
    //         ↑ 编译错误：column 'namee' not found
).fetch_all(&pool).await?;
```

## 🔍 SQLx编译时检查原理

### 检查流程

```
1. 开发者写SQL查询
   ↓
2. cargo build
   ↓  
3. SQLx宏展开
   ↓
4. 连接数据库检查
   ├── 表存在？
   ├── 字段存在？
   ├── 类型匹配？
   └── 权限足够？
   ↓
5. 生成类型安全的Rust代码
   ↓
6. 编译成功 ✅
```

### 两种工作模式

#### 在线模式 (Connected Mode)
```bash
export SQLX_OFFLINE=false
export DATABASE_URL="sqlite:data/trade_alert.db"
cargo build
```

```rust
// 编译时SQLx做这些检查：
sqlx::query!("SELECT symbol, price FROM strategy_signals")
// ↓ SQLx编译时行为：
// 1. 连接到DATABASE_URL指定的数据库
// 2. DESCRIBE strategy_signals;  
// 3. 检查symbol、price字段是否存在
// 4. 检查字段类型：TEXT、REAL等
// 5. 生成对应的Rust类型
```

#### 离线模式 (Offline Mode)
```bash
export SQLX_OFFLINE=true
cargo build
```

使用预生成的查询缓存文件：
```json
// .sqlx/query-xxx.json
{
  "query": "SELECT symbol, price FROM strategy_signals",
  "describe": {
    "columns": [
      {"name": "symbol", "type_info": "TEXT"},
      {"name": "price", "type_info": "REAL"}
    ]
  }
}
```

## 🌟 Rust独特优势

### 1. 零成本抽象
```rust
// 编译后的代码性能等同于手写SQL绑定
let users = sqlx::query!("SELECT id, name FROM users")
    .fetch_all(&pool).await?;
// ↓ 编译后等价于：
// 直接的SQL执行 + 类型转换，无运行时开销
```

### 2. 类型安全
```rust
// 其他语言
let user_id = row["id"];  // 可能是任何类型！运行时错误风险

// Rust SQLx
let user_id: i64 = row.id;  // 编译时保证是i64类型
```

### 3. 重构安全
```sql
-- 数据库改动：重命名字段
ALTER TABLE users RENAME COLUMN name TO full_name;
```

```rust
// 其他语言：运行时才发现问题
let name = row["name"];  // 💥 运行时错误

// Rust：编译时立即发现
let name = user.name;    // 💥 编译错误：field not found
//                       ✅ IDE提示：改为user.full_name
```

## 📊 TradeAlert项目中的迁移文件

```
migrations/ 目录结构
├── 20240320000000_initial.sql          ← 基础表结构 (alerts等)
├── 20250609000000_add_market_anomaly.sql ← 市场异常表
├── 20250621000000_add_stock_database.sql ← 股票数据表 (cn_stocks, us_stocks)
├── 20250712000000_add_crypto_stocks.sql  ← 加密货币表
└── 20250712000002_add_strategy_signals.sql ← YF策略信号表
```

**重要**：这些文件是"蓝图"，不是实际的数据库表！

## 🔄 每个环境都需要执行迁移

### SQLx迁移状态跟踪

SQLx自动创建状态跟踪表：
```sql
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
```

### 迁移执行过程

当运行 `sqlx migrate run` 时：

```
1. 检查 _sqlx_migrations 表
   ├── 已执行: 20240320000000_initial.sql ✅
   ├── 已执行: 20250609000000_add_market_anomaly.sql ✅  
   ├── 已执行: 20250621000000_add_stock_database.sql ✅
   ├── 已执行: 20250712000000_add_crypto_stocks.sql ✅
   └── 未执行: 20250712000002_add_strategy_signals.sql ❌

2. 发现新迁移，开始执行:
   ├── 读取 migrations/20250712000002_add_strategy_signals.sql
   ├── 执行 CREATE TABLE strategy_signals (...)
   ├── 执行 CREATE INDEX idx_strategy_signals_symbol_type (...)
   └── 记录到 _sqlx_migrations 表 ✅

3. 完成后状态:
   └── strategy_signals 表创建成功 ✅
```

## 🌍 多环境一致性保证

### 不同开发环境状态对比

```
环境A (已运行迁移):
├── migrations/ (蓝图) ✅
├── data/trade_alert.db ✅  
│   ├── alerts 表 ✅
│   ├── price_history 表 ✅
│   ├── cn_stocks 表 ✅
│   ├── us_stocks 表 ✅
│   ├── crypto_stocks 表 ✅
│   └── strategy_signals 表 ✅ ← 新建的
└── 编译状态: ✅ 成功

环境B (未运行迁移):
├── migrations/ (同样的蓝图) ✅
├── data/trade_alert.db ✅
│   ├── alerts 表 ✅
│   ├── price_history 表 ✅ 
│   ├── cn_stocks 表 ✅
│   ├── us_stocks 表 ✅
│   ├── crypto_stocks 表 ✅
│   └── strategy_signals 表 ❌ ← 缺失！
└── 编译状态: ❌ 失败
```

### 团队开发场景

```
开发者A (MacOS):           开发者B (Windows):         CI/CD服务器:
├── git pull                ├── git pull               ├── git checkout  
├── sqlx migrate run        ├── sqlx migrate run       ├── sqlx migrate run
└── cargo build ✅          └── cargo build ✅         └── docker build ✅

测试环境:                   生产环境:
├── git deploy              ├── git deploy
├── sqlx migrate run        ├── sqlx migrate run  
└── 部署成功 ✅              └── 部署成功 ✅
```

## 🚨 常见问题排查

### 编译错误：表不存在

**错误信息**：
```
error: error returned from database: (code: 1) no such table: strategy_signals
```

**原因**：本地数据库缺少新添加的表

**解决方案**：
```bash
# Windows PowerShell
$env:DATABASE_URL = "sqlite:data/trade_alert.db"
$env:SQLX_OFFLINE = "false"
sqlx migrate run

# Linux/macOS
export DATABASE_URL="sqlite:data/trade_alert.db"
export SQLX_OFFLINE="false"
sqlx migrate run
```

### 使用项目脚本

```bash
# Windows
.\scripts\dev\development\dev_migrate.ps1

# Linux/macOS  
./scripts/dev_migrate.ps1
```

## 💡 最佳实践

### 开发工作流

```bash
# 1. 拉取代码更新
git pull

# 2. 运行迁移 (确保数据库结构最新)
sqlx migrate run

# 3. 编译代码 (现在SQLx能找到所有表)
cargo build

# 4. 开发新功能...
```

### 团队协作规则

```
数据库结构变更必须通过迁移文件
├── ✅ 创建新表 → 写migration
├── ✅ 添加字段 → 写migration  
├── ✅ 创建索引 → 写migration
├── ✅ 修改字段类型 → 写migration
└── ❌ 直接改数据库 → 禁止！
```

### 迁移文件命名

```
格式: YYYYMMDDHHMMSS_description.sql
示例: 20250712000002_add_strategy_signals.sql

优势:
├── 时间戳确保执行顺序
├── 描述性名称便于理解
└── 避免迁移冲突
```

## 🔧 SQLx编译时检查的依赖关系

```rust
// 代码中的SQL查询
sqlx::query!(
    "INSERT INTO strategy_signals (symbol, strategy_type, signal_strength, ...)"
)

// SQLx编译时必须确认:
✅ strategy_signals 表存在
✅ symbol 字段存在且类型为 TEXT  
✅ strategy_type 字段存在且类型为 TEXT
✅ signal_strength 字段存在且类型为 INTEGER
❌ 如果表不存在 → 编译失败
```

## 📈 类型安全对比

| 特性 | 传统语言 | Rust SQLx |
|------|----------|-----------|
| SQL错误发现 | 运行时 | 编译时 |
| 类型安全 | 弱/无 | 强类型 |
| 重构支持 | 手动查找 | 编译器检查 |
| 性能开销 | 运行时反射 | 零成本 |
| 开发体验 | 容易出错 | IDE智能提示 |

## 🎯 总结

1. **迁移文件是蓝图**：migrations/目录包含数据库结构的版本化"建造指令"
2. **每个环境独立执行**：每个开发环境、测试环境、生产环境都需要运行相同的迁移
3. **SQLx编译时检查**：Rust独有的编译时SQL验证，确保类型安全
4. **结构一致性保证**：通过版本控制 + 迁移系统确保所有环境数据库结构一致

**核心理念**：数据库结构变更像代码一样版本化管理，确保团队协作的一致性和可靠性。