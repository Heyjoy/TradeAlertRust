#!/bin/bash

# TradeAlertRust NAS直接部署脚本
# 适用于：不使用Docker的直接部署

set -e

echo "🏠 开始NAS直接部署 TradeAlertRust..."

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# 配置
APP_DIR="/volume1/applications/trade-alert"
SERVICE_USER="trade-alert"
SERVICE_PORT="8000"

# 检查Rust环境
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}📦 安装Rust环境...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

echo -e "${GREEN}✅ Rust环境检查通过${NC}"

# 创建应用目录
echo -e "${YELLOW}📁 创建应用目录...${NC}"
sudo mkdir -p "$APP_DIR"/{data,logs,config,backups}

# 创建服务用户
if ! id "$SERVICE_USER" &>/dev/null; then
    echo -e "${YELLOW}👤 创建服务用户...${NC}"
    sudo useradd -r -s /bin/false -d "$APP_DIR" "$SERVICE_USER"
fi

# 设置权限
sudo chown -R "$SERVICE_USER:$SERVICE_USER" "$APP_DIR"

# 编译应用
echo -e "${YELLOW}🔨 编译应用...${NC}"
cargo build --release --bin trade_alert_rust

# 安装应用
echo -e "${YELLOW}📦 安装应用文件...${NC}"
sudo cp target/release/trade_alert_rust "$APP_DIR/trade_alert_rust"
sudo cp -r templates "$APP_DIR/"
sudo cp -r static "$APP_DIR/"

# 创建配置文件
CONFIG_FILE="$APP_DIR/config/app.env"
if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${YELLOW}⚙️ 创建配置文件...${NC}"
    sudo tee "$CONFIG_FILE" > /dev/null << EOF
# 服务配置
SERVER_HOST=0.0.0.0
SERVER_PORT=$SERVICE_PORT
DATABASE_URL=sqlite://$APP_DIR/data/alerts.db
RUST_LOG=info

# 邮件配置
EMAIL_SMTP_HOST=smtp.gmail.com
EMAIL_SMTP_PORT=587
EMAIL_USERNAME=your-email@gmail.com
EMAIL_PASSWORD=your-app-password
EMAIL_FROM_NAME=TradeAlert

# 日志配置
LOG_LEVEL=info
LOG_FILE=$APP_DIR/logs/app.log
EOF

    echo -e "${RED}❗ 请编辑 $CONFIG_FILE 配置你的邮件信息${NC}"
fi

# 创建systemd服务
SERVICE_FILE="/etc/systemd/system/trade-alert.service"
echo -e "${YELLOW}🔧 创建系统服务...${NC}"
sudo tee "$SERVICE_FILE" > /dev/null << EOF
[Unit]
Description=TradeAlert Rust Application
After=network.target
Wants=network.target

[Service]
Type=simple
User=$SERVICE_USER
Group=$SERVICE_USER
WorkingDirectory=$APP_DIR
ExecStart=$APP_DIR/trade_alert_rust
EnvironmentFile=$CONFIG_FILE

# 重启策略
Restart=always
RestartSec=10
StartLimitInterval=60s
StartLimitBurst=3

# 安全配置
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$APP_DIR

# 日志配置
StandardOutput=journal
StandardError=journal
SyslogIdentifier=trade-alert

[Install]
WantedBy=multi-user.target
EOF

# 重载systemd配置
sudo systemctl daemon-reload

# 启用并启动服务
echo -e "${YELLOW}🚀 启动服务...${NC}"
sudo systemctl enable trade-alert.service
sudo systemctl start trade-alert.service

# 等待服务启动
sleep 10

# 检查服务状态
if sudo systemctl is-active --quiet trade-alert.service; then
    echo -e "${GREEN}✅ 服务启动成功！${NC}"
    echo -e "${GREEN}🌐 本地访问: http://localhost:$SERVICE_PORT${NC}"
    
    # 获取内网IP
    LOCAL_IP=$(hostname -I | awk '{print $1}')
    echo -e "${GREEN}🏠 内网访问: http://$LOCAL_IP:$SERVICE_PORT${NC}"
    
    # 显示服务状态
    echo -e "\n${YELLOW}📊 服务状态:${NC}"
    sudo systemctl status trade-alert.service --no-pager -l
    
else
    echo -e "${RED}❌ 服务启动失败，检查日志：${NC}"
    sudo journalctl -u trade-alert.service --no-pager -l
    exit 1
fi

# 设置日志轮转
echo -e "${YELLOW}📝 配置日志轮转...${NC}"
sudo tee "/etc/logrotate.d/trade-alert" > /dev/null << EOF
$APP_DIR/logs/*.log {
    daily
    missingok
    rotate 30
    compress
    delaycompress
    notifempty
    create 644 $SERVICE_USER $SERVICE_USER
    postrotate
        systemctl reload trade-alert.service > /dev/null 2>&1 || true
    endscript
}
EOF

# 设置定时备份
echo -e "${YELLOW}💾 设置定时备份...${NC}"
BACKUP_SCRIPT="$APP_DIR/scripts/backup.sh"
sudo mkdir -p "$APP_DIR/scripts"
sudo tee "$BACKUP_SCRIPT" > /dev/null << 'EOF'
#!/bin/bash
# TradeAlert 数据备份脚本

DATE=$(date +%Y%m%d_%H%M%S)
APP_DIR="/volume1/applications/trade-alert"
BACKUP_DIR="$APP_DIR/backups"
DATA_DIR="$APP_DIR/data"

# 停止服务进行一致性备份
systemctl stop trade-alert.service

# 创建备份
tar -czf "$BACKUP_DIR/backup_$DATE.tar.gz" -C "$DATA_DIR" .

# 重启服务
systemctl start trade-alert.service

# 保留最近30天的备份
find "$BACKUP_DIR" -name "backup_*.tar.gz" -mtime +30 -delete

echo "$(date): 备份完成 - backup_$DATE.tar.gz"
EOF

sudo chmod +x "$BACKUP_SCRIPT"
sudo chown "$SERVICE_USER:$SERVICE_USER" "$BACKUP_SCRIPT"

# 添加到crontab
(sudo crontab -u root -l 2>/dev/null; echo "0 2 * * * $BACKUP_SCRIPT >> $APP_DIR/logs/backup.log 2>&1") | sudo crontab -u root -

# 显示管理命令
echo -e "\n${YELLOW}🛠️ 常用管理命令:${NC}"
echo "查看状态: sudo systemctl status trade-alert.service"
echo "查看日志: sudo journalctl -u trade-alert.service -f"
echo "重启服务: sudo systemctl restart trade-alert.service"
echo "停止服务: sudo systemctl stop trade-alert.service"
echo "启动服务: sudo systemctl start trade-alert.service"

echo -e "${GREEN}✅ NAS直接部署完成！${NC}"
echo -e "${GREEN}🎉 TradeAlertRust 已作为系统服务运行${NC}" 