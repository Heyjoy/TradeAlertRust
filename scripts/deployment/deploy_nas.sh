#!/bin/bash

# TradeAlertRust NAS部署脚本
# 使用说明: ./scripts/deploy_nas.sh

set -e

echo "🏠 开始NAS部署 TradeAlertRust..."

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 检查Docker是否安装
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker未安装，请先安装Docker${NC}"
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo -e "${RED}❌ docker-compose未安装，请先安装docker-compose${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Docker环境检查通过${NC}"

# 创建必要目录
echo -e "${YELLOW}📁 创建数据目录...${NC}"
sudo mkdir -p /volume1/docker/trade-alert/data
sudo mkdir -p /volume1/docker/trade-alert/logs
sudo mkdir -p /volume1/docker/trade-alert/backups

# 设置权限
sudo chown -R $USER:$USER /volume1/docker/trade-alert/
chmod 755 /volume1/docker/trade-alert/data

# 检查环境变量文件
ENV_FILE="docker/.env.nas"
if [ ! -f "$ENV_FILE" ]; then
    echo -e "${YELLOW}⚙️ 创建环境变量配置文件...${NC}"
    cat > "$ENV_FILE" << EOF
# 邮件配置
EMAIL_SMTP_HOST=smtp.gmail.com
EMAIL_SMTP_PORT=587
EMAIL_USERNAME=your-email@gmail.com
EMAIL_PASSWORD=your-app-password
EMAIL_FROM_NAME=TradeAlert

# 可选：Cloudflare配置
CLOUDFLARE_API_TOKEN=your-token
CLOUDFLARE_ZONE_ID=your-zone-id
CLOUDFLARE_DOMAIN=your-domain.com

# 监控配置
ENABLE_MONITORING=true
WEBHOOK_URL=https://hooks.slack.com/your-webhook
EOF
    echo -e "${RED}❗ 请编辑 $ENV_FILE 配置你的邮件信息${NC}"
    echo -e "${YELLOW}💡 配置完成后重新运行此脚本${NC}"
    exit 1
fi

# 构建和启动服务
echo -e "${YELLOW}🔨 构建Docker镜像...${NC}"
cd docker
docker-compose -f docker-compose.nas.yml --env-file .env.nas build

echo -e "${YELLOW}🚀 启动服务...${NC}"
docker-compose -f docker-compose.nas.yml --env-file .env.nas up -d

# 等待服务启动
echo -e "${YELLOW}⏳ 等待服务启动...${NC}"
sleep 30

# 健康检查
echo -e "${YELLOW}🏥 检查服务健康状态...${NC}"
if curl -f http://localhost:8000/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ 服务启动成功！${NC}"
    echo -e "${GREEN}🌐 本地访问: http://localhost:8000${NC}"
    
    # 获取内网IP
    LOCAL_IP=$(hostname -I | awk '{print $1}')
    echo -e "${GREEN}🏠 内网访问: http://$LOCAL_IP:8000${NC}"
    
    # 显示容器状态
    echo -e "\n${YELLOW}📊 容器状态:${NC}"
    docker-compose -f docker-compose.nas.yml ps
    
else
    echo -e "${RED}❌ 服务启动失败，检查日志：${NC}"
    docker-compose -f docker-compose.nas.yml logs --tail=50 trade-alert
    exit 1
fi

# 显示管理命令
echo -e "\n${YELLOW}🛠️ 常用管理命令:${NC}"
echo "查看日志: docker-compose -f docker-compose.nas.yml logs -f trade-alert"
echo "重启服务: docker-compose -f docker-compose.nas.yml restart trade-alert"
echo "停止服务: docker-compose -f docker-compose.nas.yml down"
echo "更新应用: ./scripts/update_nas.sh"

# 设置定时备份
echo -e "\n${YELLOW}💾 设置定时备份...${NC}"
BACKUP_SCRIPT="/volume1/docker/trade-alert/backup.sh"
cat > "$BACKUP_SCRIPT" << 'EOF'
#!/bin/bash
# TradeAlert 数据备份脚本

DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/volume1/docker/trade-alert/backups"
DATA_DIR="/volume1/docker/trade-alert/data"

# 创建备份
tar -czf "$BACKUP_DIR/backup_$DATE.tar.gz" -C "$DATA_DIR" .

# 保留最近30天的备份
find "$BACKUP_DIR" -name "backup_*.tar.gz" -mtime +30 -delete

echo "$(date): 备份完成 - backup_$DATE.tar.gz"
EOF

chmod +x "$BACKUP_SCRIPT"

# 添加到crontab (每天凌晨2点备份)
(crontab -l 2>/dev/null; echo "0 2 * * * $BACKUP_SCRIPT >> /volume1/docker/trade-alert/logs/backup.log 2>&1") | crontab -

echo -e "${GREEN}✅ NAS部署完成！${NC}"
echo -e "${GREEN}🎉 TradeAlertRust 已在你的NAS上运行${NC}" 