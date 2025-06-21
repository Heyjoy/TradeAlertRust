# 数据库迁移指南

## 🎯 简化的迁移流程

现在你只需要 3 个简单步骤就能完成数据库迁移！

### 1. 创建新迁移 📝
```powershell
.\scripts\development\new_migration.ps1 "add_user_preferences"
```
这会创建一个带时间戳的迁移文件，并提供模板。

### 2. 编辑迁移文件 ✏️
在生成的 `.sql` 文件中添加你的 SQL 语句：
```sql
-- 示例：添加用户偏好表
CREATE TABLE IF NOT EXISTS user_preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    theme TEXT DEFAULT 'light',
    language TEXT DEFAULT 'zh-CN',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_user_preferences_user_id ON user_preferences(user_id);
```

### 3. 运行迁移 🚀
```powershell
.\scripts\development\dev_migrate.ps1
```
或者直接启动开发环境（会自动运行迁移）：
```powershell
.\scripts\development\dev_start.ps1
```

## 🛠️ 可用脚本

| 脚本 | 用途 | 示例 |
|------|------|------|
| `new_migration.ps1` | 创建新迁移文件 | `.\scripts\development\new_migration.ps1 "add_notifications"` |
| `dev_migrate.ps1` | 运行数据库迁移 | `.\scripts\development\dev_migrate.ps1` |
| `dev_start.ps1` | 启动开发环境 | `.\scripts\development\dev_start.ps1` |

## 📋 最佳实践

### ✅ 推荐做法
- 使用描述性的迁移名称：`add_user_table`, `update_price_index`
- 总是使用 `CREATE TABLE IF NOT EXISTS`
- 为经常查询的字段创建索引
- 在迁移中添加注释说明用途

### ❌ 避免的问题
- 不要直接修改已应用的迁移文件
- 不要在生产环境删除数据库文件
- 避免在迁移中使用复杂的触发器（SQLite 兼容性问题）

## 🔧 故障排除

### 问题：`no such table` 错误
**解决方案：**
```powershell
# 重新运行迁移
.\scripts\development\dev_migrate.ps1
```

### 问题：SQLx 离线模式错误
**解决方案：**
```powershell
# 更新查询缓存
$env:DATABASE_URL="sqlite:data/trade_alert.db"
cargo sqlx prepare --workspace
```

### 问题：迁移冲突
**解决方案：**
1. 检查 `migrations/` 目录中的文件顺序
2. 确保文件名时间戳正确
3. 如果是开发环境，可以删除 `data/trade_alert.db` 重新开始

## 🎉 现在开发更简单了！

以前需要：
1. 手动创建迁移文件
2. 设置环境变量
3. 运行 sqlx migrate
4. 处理各种错误
5. 更新查询缓存
6. 启动应用

现在只需要：
1. `.\scripts\development\new_migration.ps1 "feature_name"`
2. 编辑 SQL 文件
3. `.\scripts\development\dev_start.ps1`

就这么简单！🚀 