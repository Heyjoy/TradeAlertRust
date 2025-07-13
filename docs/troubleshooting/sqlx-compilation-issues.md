# SQLx 编译问题解决指南

本文档详细说明了在 TradeAlert 项目中遇到 SQLx 编译错误的常见原因和解决方案。

## 常见错误类型

### 1. 查询缓存缺失错误

**错误信息：**
```
error: `SQLX_OFFLINE=true` but there is no cached data for this query, run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
```

**原因：**
- SQLx 在离线模式下需要预先生成的查询缓存
- 新增或修改了数据库查询，但未更新缓存

**解决方案：**
```bash
# 1. 设置必要的环境变量
export DATABASE_URL="sqlite:data/trade_alert.db"
export SQLX_OFFLINE=false

# 2. 更新查询缓存
cargo sqlx prepare

# 3. 重新编译
cargo build
```

### 2. 类型转换错误

**错误信息：**
```
error[E0277]: the trait bound `String: From<Option<String>>` is not satisfied
```

**原因：**
- 数据库字段是 nullable（通过 ALTER TABLE ADD COLUMN 添加）
- Rust 模型中定义为非 nullable 类型
- SQLx 推断字段类型为 `Option<T>`，但模型期望 `T`

**解决方案：**

#### 方法1：使用 COALESCE 提供默认值
```rust
// 错误的查询
sqlx::query_as!(
    Alert,
    "SELECT id, symbol, user_id FROM alerts WHERE id = ?",
    id
)

// 正确的查询
sqlx::query_as!(
    Alert,
    r#"
    SELECT id as "id!", symbol, 
           COALESCE(user_id, 'default') as "user_id!"
    FROM alerts WHERE id = ?
    "#,
    id
)
```

#### 方法2：修改模型定义
```rust
// 如果字段确实可以为空，修改模型
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Alert {
    pub id: i64,
    pub symbol: String,
    pub user_id: Option<String>, // 改为 Option<String>
}
```

### 3. Rust 环境配置问题

**错误信息：**
```
/bin/bash: line 1: cargo: command not found
```

**原因：**
- Rust 未安装或未添加到 PATH

**解决方案：**
```bash
# 1. 检查 Rust 是否安装
which cargo

# 2. 如果在 ~/.cargo/bin/ 中存在，添加到 PATH
export PATH="$HOME/.cargo/bin:$PATH"

# 3. 永久配置（选择适合的 shell 配置文件）
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
# 或
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc

# 4. 重新加载配置
source ~/.bashrc
# 或
source ~/.zshrc
```

### 4. 参数绑定错误

**错误信息：**
```
error: expected 0 parameters, got 1
```

**原因：**
- SQL 查询中使用了字符串插值而不是参数绑定
- 动态 SQL 构造错误

**解决方案：**
```rust
// 错误的方式
sqlx::query!(
    "DELETE FROM alerts WHERE created_at < datetime('now', '-{} hours')",
    retention_hours
)

// 正确的方式
let retention_hours_str = retention_hours.to_string();
sqlx::query!(
    "DELETE FROM alerts WHERE created_at < datetime('now', '-' || ? || ' hours')",
    retention_hours_str
)
```

## 预防措施

### 1. 开发流程最佳实践

```bash
# 修改数据库查询后，立即更新缓存
cargo sqlx prepare

# 定期检查查询缓存是否同步
cargo check

# 提交代码前确保包含 .sqlx 目录
git add .sqlx/
git commit -m "Update SQLx query cache"
```

### 2. 数据库迁移最佳实践

```sql
-- 添加新字段时，提供默认值
ALTER TABLE alerts ADD COLUMN user_id TEXT DEFAULT 'default';

-- 而不是
ALTER TABLE alerts ADD COLUMN user_id TEXT; -- 这会导致 nullable 字段
```

### 3. 查询编写最佳实践

```rust
// 明确指定字段类型，特别是可能为 null 的字段
sqlx::query_as!(
    Alert,
    r#"
    SELECT 
        id as "id!",
        symbol,
        COALESCE(user_id, 'default') as "user_id!",
        notification_email  -- 保持 Option<String>
    FROM alerts 
    WHERE id = ?
    "#,
    id
)
```

## 调试工具

### 1. 查看生成的查询缓存
```bash
# 查看缓存的查询信息
cat .sqlx/query-*.json | jq '.'
```

### 2. 验证数据库连接
```bash
# 使用 SQLx CLI 测试连接
sqlx database create
sqlx migrate run
```

### 3. 检查表结构
```bash
sqlite3 data/trade_alert.db ".schema alerts"
```

## 故障排除检查清单

当遇到 SQLx 编译错误时，按以下顺序检查：

- [ ] Rust 环境是否正确配置（`cargo --version`）
- [ ] DATABASE_URL 环境变量是否设置
- [ ] 数据库文件是否存在且可访问
- [ ] 数据库迁移是否已应用（`sqlx migrate info`）
- [ ] 查询缓存是否最新（`cargo sqlx prepare`）
- [ ] 查询中的类型注解是否正确
- [ ] nullable 字段是否正确处理

## 相关资源

- [SQLx 官方文档](https://docs.rs/sqlx/)
- [SQLx GitHub 仓库](https://github.com/launchbadge/sqlx)
- [项目数据库迁移指南](../DATABASE_MIGRATION_GUIDE.md)