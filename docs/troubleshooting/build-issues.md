# 🔨 编译和构建问题解决方案

## 目录
- [SQLX离线模式错误](#sqlx离线模式错误)
- [Rust编译错误](#rust编译错误)
- [依赖包问题](#依赖包问题)
- [生命周期错误](#生命周期错误)

---

## SQLX离线模式错误 {#sqlx-offline-error}

### 问题症状
```
error: `SQLX_OFFLINE=true` but there is no cached data for this query, 
run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
```

### 原因分析
- 新增的SQL查询没有缓存数据
- SQLX离线模式启用但缺少查询缓存
- 数据库schema发生变化

### 🔧 解决方案

#### 方案1: 禁用离线模式（推荐）
```powershell
# 临时禁用SQLX离线模式
$env:SQLX_OFFLINE="false"

# 重新编译
cargo build

# 启动服务器
cargo run
```

#### 方案2: 生成查询缓存
```powershell
# 确保数据库存在
cargo run --bin migrate

# 生成SQLX查询缓存
cargo sqlx prepare

# 启用离线模式编译
$env:SQLX_OFFLINE="true"
cargo build
```

#### 方案3: 删除现有缓存重新生成
```powershell
# 删除现有缓存文件
rm .sqlx/ -r

# 禁用离线模式
$env:SQLX_OFFLINE="false"

# 重新生成缓存
cargo sqlx prepare

# 编译项目
cargo build
```

### 预防措施
- 开发环境建议使用 `SQLX_OFFLINE=false`
- 生产环境使用离线模式提高构建速度
- 每次修改SQL查询后运行 `cargo sqlx prepare`

---

## Rust编译错误 {#rust-compile-errors}

### 生命周期错误

#### 问题症状
```
error[E0716]: temporary value dropped while borrowed
   --> src/main.rs:630:9
    |
630 |         format!("%{}%", query),
    |         ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
```

#### 解决方案
```rust
// ❌ 错误写法
let results = sqlx::query!(
    "SELECT * FROM table WHERE name LIKE ?",
    format!("%{}%", query)  // 临时值会被释放
);

// ✅ 正确写法
let search_pattern = format!("%{}%", query);
let results = sqlx::query!(
    "SELECT * FROM table WHERE name LIKE ?",
    search_pattern  // 使用变量存储
);
```

### 未使用导入警告

#### 问题症状
```
warning: unused import: `State`
 --> src\handlers\market.rs:2:21
  |
2 |     extract::{Path, State}, 
  |                     ^^^^^
```

#### 解决方案
```rust
// 删除未使用的导入
use axum::{
    extract::Path,  // 删除 State
    response::Html,
};

// 或者使用 #[allow] 属性
#[allow(unused_imports)]
use axum::extract::State;
```

### 类型不匹配错误

#### 问题症状
```
error[E0308]: mismatched types
expected `String`, found `&str`
```

#### 解决方案
```rust
// ❌ 类型不匹配
let name: String = "hello";  // &str 赋给 String

// ✅ 正确转换
let name: String = "hello".to_string();
let name: String = String::from("hello");
let name = "hello".to_owned();
```

---

## 依赖包问题 {#dependency-issues}

### 版本冲突

#### 问题症状
```
error: failed to select a version for the requirement `tokio = "^1.0"`
```

#### 解决方案
```powershell
# 更新依赖版本
cargo update

# 检查依赖树
cargo tree

# 清理并重新获取依赖
cargo clean
rm Cargo.lock
cargo build
```

### 缺少系统依赖

#### 问题症状 (Windows)
```
error: linking with `link.exe` failed
```

#### 解决方案
```powershell
# 安装Visual Studio Build Tools
# 或者安装Visual Studio Community

# 检查Rust工具链
rustup show

# 更新工具链
rustup update
```

### SQLite相关错误

#### 问题症状
```
error: failed to run custom build command for `libsqlite3-sys`
```

#### 解决方案
```powershell
# Windows: 确保有C++构建工具
# 安装 vcpkg 或使用捆绑的SQLite
$env:SQLX_FEATURES="runtime-tokio-rustls,sqlite,bundled"

# 重新编译
cargo clean
cargo build
```

---

## 模板引擎问题 {#template-issues}

### Askama模板错误

#### 问题症状
```
error: failed to derive `Template` for `IndexTemplate`
```

#### 解决方案
```rust
// 确保模板文件存在于正确位置
// templates/index.html

// 检查模板语法
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub alerts: Vec<Alert>,
}
```

### JavaScript语法冲突

#### 问题症状
模板中的JavaScript箭头函数导致编译错误

#### 解决方案
```html
<!-- ❌ 可能导致问题的写法 -->
<script>
const func = () => {
    // 箭头函数可能与模板引擎冲突
};
</script>

<!-- ✅ 推荐写法 -->
<script>
function func() {
    // 使用传统函数声明
}

// 或者使用外部JS文件
</script>
<script src="/static/js/app.js"></script>
```

---

## 编译优化建议 {#compile-optimization}

### 加速编译

#### 并行编译
```powershell
# 设置并行编译任务数
$env:CARGO_BUILD_JOBS="8"

# 使用sccache缓存
cargo install sccache
$env:RUSTC_WRAPPER="sccache"
```

#### 增量编译
```powershell
# 启用增量编译（默认启用）
$env:CARGO_INCREMENTAL="1"

# 检查编译缓存
cargo clean --doc
cargo clean --release
```

### 调试构建

#### 详细错误信息
```powershell
# 显示详细编译信息
cargo build --verbose

# 显示编译时间
cargo build --timings

# 检查代码但不生成可执行文件
cargo check
```

---

## 常见编译脚本 {#build-scripts}

### 一键构建脚本
```powershell
# scripts/build.ps1
Write-Host "🔨 开始构建 TradeAlert..." -ForegroundColor Cyan

# 设置环境变量
$env:SQLX_OFFLINE="false"
$env:RUST_LOG="info"

# 清理旧的构建
Write-Host "🧹 清理构建缓存..."
cargo clean

# 检查代码
Write-Host "🔍 检查代码语法..."
$checkResult = cargo check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 代码检查失败" -ForegroundColor Red
    exit 1
}

# 运行数据库迁移
Write-Host "💾 运行数据库迁移..."
cargo run --bin migrate

# 构建项目
Write-Host "🔨 构建项目..."
cargo build

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ 构建成功！" -ForegroundColor Green
} else {
    Write-Host "❌ 构建失败" -ForegroundColor Red
    exit 1
}
```

### 开发环境快速重建
```powershell
# scripts/dev-rebuild.ps1
Write-Host "⚡ 快速重建开发环境..." -ForegroundColor Yellow

# 杀死现有进程
taskkill /f /im trade_alert_rust.exe 2>$null

# 设置环境变量
$env:SQLX_OFFLINE="false"

# 快速编译检查
cargo check --bin trade_alert_rust

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ 编译检查通过，启动服务器..." -ForegroundColor Green
    cargo run
} else {
    Write-Host "❌ 编译失败，请检查错误信息" -ForegroundColor Red
}
```

---

## 问题诊断清单

### 编译前检查
- [ ] 检查Rust版本 (`rustc --version`)
- [ ] 检查Cargo版本 (`cargo --version`)
- [ ] 检查环境变量设置
- [ ] 确认数据库文件存在
- [ ] 检查依赖版本兼容性

### 编译时检查
- [ ] 使用 `cargo check` 快速检查语法
- [ ] 查看详细错误信息
- [ ] 检查SQLX相关配置
- [ ] 确认模板文件路径正确

### 编译后检查
- [ ] 验证可执行文件生成
- [ ] 检查数据库迁移状态
- [ ] 测试基本功能
- [ ] 查看运行时日志

**预估解决时间**: 5-15分钟  
**难度等级**: 🟡 中等 