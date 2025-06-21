# ARM 架构 Synology NAS 部署指南

## 🚨 重要说明
您的 Synology 设备是 ARM 架构，官方不提供 Docker 套件支持。我们提供两种部署方案：

## 📋 方案选择

### 方案1：手动安装 Docker（推荐但有风险）
- ✅ 可以使用容器化部署
- ⚠️ 完全不受 Synology 官方支持
- ⚠️ 有可能损坏系统
- ✅ 功能完整

### 方案2：原生 Rust 应用部署（安全推荐）
- ✅ 官方支持，系统安全
- ✅ 性能更好
- ❌ 需要手动编译
- ✅ 资源占用小

---

## 🐳 方案1：手动安装 Docker

### ⚠️ 风险警告
> **此方法完全不受 Synology 官方支持，可能会损坏您的 NAS！**
> 
> 请确保重要数据已备份后再继续！

### 第一步：确认设备架构
SSH 连接到您的 NAS：
```bash
ssh your-username@your-nas-ip
uname -m
```

输出应该是：
- `armv7l` - 32位 ARM
- `aarch64` - 64位 ARM

### 第二步：安装 Docker
根据您的架构选择对应脚本：

**32位 ARM (armv7l):**
```bash
# 切换到 root 用户
sudo -i

# 下载安装脚本
curl https://gist.githubusercontent.com/ta264/2b7fb6e6466b109b9bf9b0a1d91ebedc/raw/b76a28d25d0abd0d27a0c9afaefa0d499eb87d3d/get-docker.sh | sh
```

**64位 ARM (aarch64):**
```bash
# 切换到 root 用户
sudo -i

# 下载专用安装脚本
wget https://github.com/ypkdani00/docker-on-synology-arm64/raw/main/install.sh
chmod +x install.sh
./install.sh
```

### 第三步：配置 Docker
```bash
# 创建数据目录
mkdir -p /volume1/@docker

# 创建配置文件
mkdir -p /etc/docker
cat > /etc/docker/daemon.json << 'EOF'
{
  "storage-driver": "vfs",
  "iptables": false,
  "data-root": "/volume1/@docker"
}
EOF

# 启动 Docker
dockerd &
```

### 第四步：设置开机自启
1. 打开 DSM → 控制面板 → 任务计划
2. 新增 → 触发的任务 → 用户定义的脚本
3. 配置：
   - 任务：Docker启动
   - 用户：root
   - 事件：开机
   - 运行命令：`dockerd &`

### 第五步：部署 TradeAlert
创建专用的 ARM docker-compose 文件：

```yaml
version: '3.3'

services:
  trade-alert:
    image: rust:1.60-slim
    container_name: trade-alert-rust
    restart: unless-stopped
    network_mode: host  # ARM 架构需要
    
    environment:
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
      - DATABASE_URL=sqlite:///app/data/alerts.db
      - RUST_LOG=info
      - TZ=Asia/Shanghai
      
      # 邮件配置
      - EMAIL_SMTP_HOST=smtp.gmail.com
      - EMAIL_SMTP_PORT=587
      - EMAIL_USERNAME=your-email@gmail.com
      - EMAIL_PASSWORD=your-app-password
      - EMAIL_FROM_NAME=TradeAlert
    
    volumes:
      - /volume1/trade-alert/source:/app/source
      - /volume1/trade-alert/data:/app/data
      - /volume1/trade-alert/logs:/app/logs
    
    working_dir: /app/source
    command: >
      bash -c "
      apt-get update && 
      apt-get install -y pkg-config libssl-dev ca-certificates curl && 
      cargo build --release --bin trade_alert_rust && 
      mkdir -p /app/data && 
      ./target/release/trade_alert_rust
      "

  redis:
    image: redis:5-alpine  # ARM 兼容版本
    container_name: trade-alert-redis
    restart: unless-stopped
    network_mode: host
    
    volumes:
      - /volume1/trade-alert/redis:/data
```

---

## 🦀 方案2：原生 Rust 应用部署（推荐）

### 第一步：准备环境
```bash
# SSH 连接到 NAS
ssh your-username@your-nas-ip

# 创建项目目录
mkdir -p /volume1/trade-alert/{source,data,logs,config}
```

### 第二步：上传项目源码
将整个 `TradeAlertRust` 项目上传到：
`/volume1/trade-alert/source/`

### 第三步：安装 Rust
```bash
# 切换到项目目录
cd /volume1/trade-alert/source

# 安装 Rust（如果没有）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 检查是否支持交叉编译
rustc --version
```

### 第四步：安装系统依赖
```bash
# Synology 通常基于 Linux，安装编译依赖
# 这一步可能需要 entware 或 SynoCLI
# 如果没有包管理器，可能需要跳过此方案

# 尝试安装必要包
opkg update
opkg install gcc
opkg install openssl-dev
opkg install pkg-config
```

### 第五步：编译应用
```bash
cd /volume1/trade-alert/source

# 编译
cargo build --release --bin trade_alert_rust

# 检查编译结果
ls -la target/release/trade_alert_rust
```

### 第六步：创建启动脚本
```bash
cat > /volume1/trade-alert/start.sh << 'EOF'
#!/bin/bash

export SERVER_HOST=0.0.0.0
export SERVER_PORT=8000
export DATABASE_URL=sqlite:///volume1/trade-alert/data/alerts.db
export RUST_LOG=info
export TZ=Asia/Shanghai

# 邮件配置 - 请修改
export EMAIL_SMTP_HOST=smtp.gmail.com
export EMAIL_SMTP_PORT=587
export EMAIL_USERNAME=your-email@gmail.com
export EMAIL_PASSWORD=your-app-password
export EMAIL_FROM_NAME=TradeAlert

cd /volume1/trade-alert/source
./target/release/trade_alert_rust
EOF

chmod +x /volume1/trade-alert/start.sh
```

### 第七步：设置开机自启
1. DSM → 控制面板 → 任务计划
2. 新增 → 触发的任务 → 用户定义的脚本
3. 配置：
   - 任务：TradeAlert启动
   - 用户：root
   - 事件：开机
   - 运行命令：`/volume1/trade-alert/start.sh`

---

## 🔧 方案3：简化部署（如果无法编译）

如果无法在 ARM 设备上编译，可以考虑：

### 使用预编译二进制
```bash
# 在 x86_64 Linux 机器上交叉编译 ARM 版本
rustup target add armv7-unknown-linux-gnueabihf  # 32位ARM
rustup target add aarch64-unknown-linux-gnu       # 64位ARM

# 编译 ARM 版本
cargo build --release --target armv7-unknown-linux-gnueabihf --bin trade_alert_rust

# 将编译好的二进制文件上传到 NAS
```

### 使用 qemu 用户模式模拟
```bash
# 在 NAS 上安装 qemu（如果可能）
# 这样可以运行 x86_64 二进制文件
```

---

## 📋 推荐方案选择

### 如果您是技术专家且接受风险：
👉 **方案1：手动安装 Docker**

### 如果您希望系统稳定安全：
👉 **方案2：原生 Rust 部署**

### 如果技术能力有限：
👉 考虑使用其他设备（如树莓派）或升级到 x86_64 架构的 Synology

---

## ❗ 重要注意事项

1. **备份数据**：任何操作前都要备份重要数据
2. **系统稳定性**：ARM 设备资源有限，运行复杂应用可能影响 NAS 基本功能
3. **官方支持**：这些方案都不受 Synology 官方支持
4. **性能考虑**：ARM 设备性能有限，建议监控系统资源使用

您想尝试哪个方案？我可以为您提供详细的操作指导。 