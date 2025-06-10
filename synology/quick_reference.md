# 🚀 ARM Synology 安全部署 - 快速参考

## 📋 部署总览
1. **Synology 权限配置** (5分钟)
2. **Windows 交叉编译** (10分钟)
3. **上传到 NAS** (2分钟)
4. **NAS 安全配置** (10分钟)
5. **自动启动设置** (5分钟)

## 🔧 Synology 必需权限

### SSH 设置
```
DSM → 控制面板 → 终端机和 SNMP → 终端机
☑️ 启动 SSH 功能
```

### 防火墙配置
```
DSM → 控制面板 → 安全性 → 防火墙
新增规则：SSH (22) - 仅允许内网IP
```

### 创建用户
```
DSM → 控制面板 → 用户账号 → 新增
用户名：tradealert
权限：users 群组，可SSH访问
```

## 💻 Windows 快速编译

### 一键启动
```cmd
deploy_to_synology_arm.bat
```

### 手动编译
```powershell
# 安装工具
cargo install cross

# 编译 (选择对应架构)
cross build --release --target armv7-unknown-linux-gnueabihf --bin trade_alert_rust
cross build --release --target aarch64-unknown-linux-gnu --bin trade_alert_rust
```

## 📤 上传命令

### 32位 ARM
```powershell
scp target\armv7-unknown-linux-gnueabihf\release\trade_alert_rust tradealert@NAS_IP:/volume1/apps/trade-alert/
```

### 64位 ARM
```powershell
scp target\aarch64-unknown-linux-gnu\release\trade_alert_rust tradealert@NAS_IP:/volume1/apps/trade-alert/
```

### 静态文件
```powershell
scp -r templates tradealert@NAS_IP:/volume1/apps/trade-alert/
scp -r static tradealert@NAS_IP:/volume1/apps/trade-alert/
```

## 🔒 NAS 安全配置

### SSH 连接
```bash
ssh tradealert@NAS_IP
```

### 目录权限
```bash
cd /volume1/apps/trade-alert
mkdir -p {data,logs,config,backup}
chmod +x trade_alert_rust
chmod 700 data logs backup
chmod 755 config
```

### 配置文件
```bash
cat > config/app.env << 'EOF'
SERVER_HOST=127.0.0.1
SERVER_PORT=8000
DATABASE_URL=sqlite:///volume1/apps/trade-alert/data/alerts.db
RUST_LOG=info

# 邮件配置 - 请修改
EMAIL_SMTP_HOST=smtp.gmail.com
EMAIL_SMTP_PORT=587
EMAIL_USERNAME=your-email@gmail.com
EMAIL_PASSWORD=your-gmail-app-password
EMAIL_FROM_NAME=TradeAlert

TZ=Asia/Shanghai
EOF

chmod 600 config/app.env
```

### 启动脚本
```bash
cat > start.sh << 'EOF'
#!/bin/bash
if [ "$(id -u)" = "0" ]; then
    echo "错误：不应该以root用户运行"
    exit 1
fi

cd /volume1/apps/trade-alert
set -a
source config/app.env
set +a

echo "$(date): 启动 TradeAlert" >> logs/app.log
./trade_alert_rust 2>&1 | tee -a logs/app.log
EOF

chmod +x start.sh
```

## 🚀 自动启动

### DSM 任务计划
```
DSM → 控制面板 → 任务计划 → 新增

开机任务：
- 任务：TradeAlert启动
- 用户：tradealert
- 事件：开机
- 命令：/volume1/apps/trade-alert/start.sh

监控任务：
- 任务：TradeAlert监控
- 用户：tradealert
- 计划：每5分钟
- 命令：监控脚本
```

## 🌐 访问地址

### 内网访问
```
http://NAS_IP:8000
```

### 反向代理（外网）
```
DSM → 控制面板 → 应用程序门户 → 反向代理
来源：HTTPS (443) → 目标：127.0.0.1:8000
```

## 🔍 故障排除

### 检查服务状态
```bash
ps aux | grep trade_alert_rust
```

### 查看日志
```bash
tail -f /volume1/apps/trade-alert/logs/app.log
```

### 测试网络
```bash
netstat -tuln | grep 8000
```

### 重启服务
```bash
pkill trade_alert_rust
cd /volume1/apps/trade-alert && ./start.sh &
```

## 📞 需要帮助？

- 📖 详细指南：`synology/step_by_step_secure_deployment.md`
- 🛡️ 安全分析：`synology/security_risks_analysis.md`
- 🔧 安全配置：`synology/secure_native_deployment.md`

## ✅ 安全检查清单

- [ ] SSH 仅内网访问
- [ ] 专用用户运行
- [ ] 服务绑定本地IP
- [ ] 配置文件权限 600
- [ ] 目录权限隔离
- [ ] 防火墙规则配置
- [ ] 邮件配置正确
- [ ] 自动启动工作
- [ ] 监控脚本运行
- [ ] 定期备份设置 