# 最安全的原生部署方案

## 🛡️ 安全优先的部署策略

基于您对安全的关注，强烈推荐使用**原生 Rust 部署**，完全避免 Docker 相关风险。

## 📋 安全原生部署方案

### 第一步：创建隔离环境
```bash
# 创建专用用户（可选但推荐）
sudo adduser tradealert
sudo usermod -a -G users tradealert

# 创建隔离的项目目录
sudo mkdir -p /volume1/apps/trade-alert
sudo chown tradealert:users /volume1/apps/trade-alert
sudo chmod 750 /volume1/apps/trade-alert
```

### 第二步：安全的文件结构
```
/volume1/apps/trade-alert/
├── bin/                    # 可执行文件
├── data/                   # 数据库（只有应用可访问）
├── logs/                   # 日志文件
├── config/                 # 配置文件
└── backup/                 # 备份文件
```

### 第三步：编译安全版本
```bash
# 在开发机器上交叉编译（推荐）
rustup target add armv7-unknown-linux-gnueabihf  # 32位ARM
rustup target add aarch64-unknown-linux-gnu       # 64位ARM

# 编译 ARM 版本
cargo build --release --target aarch64-unknown-linux-gnu --bin trade_alert_rust

# 上传到 NAS
scp target/aarch64-unknown-linux-gnu/release/trade_alert_rust user@nas:/volume1/apps/trade-alert/bin/
```

### 第四步：安全配置
```bash
# 创建配置文件
cat > /volume1/apps/trade-alert/config/app.env << 'EOF'
# 网络安全配置
SERVER_HOST=127.0.0.1  # 只绑定本地，不暴露到网络
SERVER_PORT=8000

# 数据库配置
DATABASE_URL=sqlite:///volume1/apps/trade-alert/data/alerts.db

# 日志配置
RUST_LOG=info

# 邮件配置（请修改）
EMAIL_SMTP_HOST=smtp.gmail.com
EMAIL_SMTP_PORT=587
EMAIL_USERNAME=your-email@gmail.com
EMAIL_PASSWORD=your-app-password
EMAIL_FROM_NAME=TradeAlert

# 安全配置
TZ=Asia/Shanghai
EOF

# 设置安全权限
chmod 600 /volume1/apps/trade-alert/config/app.env
```

### 第五步：创建安全启动脚本
```bash
cat > /volume1/apps/trade-alert/start.sh << 'EOF'
#!/bin/bash

# 安全检查
if [ "$(id -u)" = "0" ]; then
    echo "错误：不应该以root用户运行"
    exit 1
fi

# 加载环境变量
set -a
source /volume1/apps/trade-alert/config/app.env
set +a

# 切换到应用目录
cd /volume1/apps/trade-alert

# 创建必要目录
mkdir -p data logs backup

# 设置文件权限
chmod 700 data logs backup

# 启动应用
exec ./bin/trade_alert_rust
EOF

chmod 755 /volume1/apps/trade-alert/start.sh
```

### 第六步：防火墙配置
```bash
# 确保只有内网可以访问
# 在群晖防火墙中设置规则：
# 允许：内网IP段 → 端口8000
# 拒绝：外网 → 端口8000
```

## 🔒 安全特性

### 1. 网络隔离
- 只绑定本地IP (127.0.0.1)
- 通过反向代理访问（可选）
- 防火墙限制外网访问

### 2. 文件系统安全
- 专用目录，权限隔离
- 配置文件加密存储
- 敏感数据权限保护

### 3. 运行时安全
- 非root用户运行
- 资源使用监控
- 异常自动重启

## 🌐 安全的外网访问（可选）

如果需要外网访问，推荐使用 Synology 的反向代理：

### 1. 配置反向代理
```
DSM → 控制面板 → 应用程序门户 → 反向代理
- 来源：your-domain.synology.me:443
- 目标：127.0.0.1:8000
```

### 2. SSL证书
- 使用 Let's Encrypt 免费证书
- 启用 HTTPS 加密

### 3. 访问控制
- IP白名单
- VPN访问
- 双因素认证

## 📊 安全监控

### 1. 系统监控脚本
```bash
cat > /volume1/apps/trade-alert/monitor.sh << 'EOF'
#!/bin/bash

# 检查进程是否运行
if ! pgrep -f trade_alert_rust > /dev/null; then
    echo "$(date): 应用未运行，尝试重启" >> logs/monitor.log
    ./start.sh &
fi

# 检查端口是否被异常占用
if netstat -tuln | grep -q ":8000.*LISTEN"; then
    PORT_PROCESS=$(netstat -tulpn | grep ":8000.*LISTEN" | awk '{print $7}')
    if [[ ! "$PORT_PROCESS" =~ trade_alert_rust ]]; then
        echo "$(date): 端口8000被未知进程占用: $PORT_PROCESS" >> logs/security.log
    fi
fi

# 检查系统资源
CPU_USAGE=$(top -bn1 | grep "trade_alert_rust" | awk '{print $9}')
if [ "${CPU_USAGE%.*}" -gt 80 ]; then
    echo "$(date): CPU使用率过高: $CPU_USAGE%" >> logs/performance.log
fi
EOF

chmod +x /volume1/apps/trade-alert/monitor.sh
```

### 2. 定时任务
```
DSM → 控制面板 → 任务计划
- 新增 → 计划的任务 → 用户定义的脚本
- 任务：TradeAlert监控
- 用户：tradealert
- 计划：每5分钟
- 运行命令：/volume1/apps/trade-alert/monitor.sh
```

## 🚨 安全检查清单

### 部署前检查
- [ ] 创建专用用户
- [ ] 设置目录权限
- [ ] 配置防火墙
- [ ] 备份重要数据
- [ ] 测试回滚方案

### 运行时检查
- [ ] 进程运行状态
- [ ] 网络端口监听
- [ ] 文件权限正确
- [ ] 日志正常记录
- [ ] 资源使用合理

### 定期安全审计
- [ ] 检查异常登录
- [ ] 查看访问日志
- [ ] 监控系统资源
- [ ] 更新安全补丁
- [ ] 验证备份完整

## 🎯 推荐配置

### 最安全配置
```bash
# 只允许本地访问
SERVER_HOST=127.0.0.1
# 使用非标准端口
SERVER_PORT=18000
# 启用详细日志
RUST_LOG=debug
```

### 通过VPN访问
如果需要远程访问：
1. 配置 Synology VPN Server
2. 通过VPN连接后访问内网IP
3. 避免直接暴露到公网

## 📞 技术支持

如果您选择此方案，我可以：
1. 提供详细的逐步操作指导
2. 帮助配置交叉编译环境
3. 协助安全配置和监控设置
4. 提供故障排除支持

这个方案完全避免了Docker的安全风险，是ARM架构Synology最安全的部署选择。 