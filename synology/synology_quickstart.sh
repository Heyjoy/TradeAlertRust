#!/bin/bash

# Synology NAS 快速部署 TradeAlertRust
# 使用方法: ssh到NAS后运行此脚本

set -e

echo "🏠 Synology NAS 快速部署 TradeAlertRust"
echo "================================================"

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# 检测Synology环境
if [ ! -d "/volume1" ]; then
    echo -e "${RED}❌ 未检测到Synology环境！${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 检测到Synology环境${NC}"

# 配置变量
PROJECT_DIR="/volume1/docker/trade-alert"
DOCKER_IMAGE="trade-alert-rust:latest"

# 检查Docker是否可用
if ! command -v docker &> /dev/null; then
    echo -e "${YELLOW}📦 请先在DSM套件中心安装 Container Manager${NC}"
    echo "路径: 套件中心 → 搜索 'Container Manager' → 安装"
    exit 1
fi

echo -e "${GREEN}✅ Docker环境检查通过${NC}"

# 创建项目目录结构
echo -e "${YELLOW}📁 创建项目目录...${NC}"
sudo mkdir -p "$PROJECT_DIR"/{data,logs,config,backups,scripts,redis}

# 设置目录权限
sudo chmod 755 "$PROJECT_DIR"
sudo chown -R $(whoami):users "$PROJECT_DIR"

echo -e "${GREEN}✅ 目录结构创建完成${NC}"

# 创建环境变量文件
ENV_FILE="$PROJECT_DIR/.env"
if [ ! -f "$ENV_FILE" ]; then
    echo -e "${YELLOW}⚙️ 创建环境变量配置...${NC}"
    cat > "$ENV_FILE" << 'EOF'
# 邮件配置 - 请修改为你的实际配置
EMAIL_SMTP_HOST=smtp.gmail.com
EMAIL_SMTP_PORT=587
EMAIL_USERNAME=your-email@gmail.com
EMAIL_PASSWORD=your-app-password
EMAIL_FROM_NAME=TradeAlert

# 时区设置
TZ=Asia/Shanghai

# 数据库配置
DATABASE_URL=sqlite:///app/data/alerts.db

# 应用配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8000
RUST_LOG=info
EOF

    echo -e "${RED}❗ 请编辑环境变量文件: $ENV_FILE${NC}"
    echo -e "${YELLOW}💡 特别注意配置邮件相关参数${NC}"
    echo ""
    echo -e "${BLUE}📝 要现在编辑吗？(y/n)${NC}"
    read -r edit_now
    
    if [[ $edit_now =~ ^[Yy]$ ]]; then
        nano "$ENV_FILE" || vi "$ENV_FILE"
    else
        echo -e "${YELLOW}⚠️ 请稍后手动编辑配置文件${NC}"
    fi
fi

# 复制docker-compose文件
echo -e "${YELLOW}📋 创建docker-compose配置...${NC}"
cat > "$PROJECT_DIR/docker-compose.yml" << 'EOF'
version: '3.8'

services:
  trade-alert:
    image: trade-alert-rust:latest
    container_name: trade-alert-rust
    restart: unless-stopped
    
    ports:
      - "8000:8000"
    
    environment:
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
      - DATABASE_URL=sqlite:///app/data/alerts.db
      - RUST_LOG=${RUST_LOG:-info}
      - EMAIL_SMTP_HOST=${EMAIL_SMTP_HOST}
      - EMAIL_SMTP_PORT=${EMAIL_SMTP_PORT}
      - EMAIL_USERNAME=${EMAIL_USERNAME}
      - EMAIL_PASSWORD=${EMAIL_PASSWORD}
      - EMAIL_FROM_NAME=${EMAIL_FROM_NAME}
      - TZ=${TZ:-Asia/Shanghai}
    
    volumes:
      - ./data:/app/data
      - ./logs:/app/logs
      - ./config:/app/config
    
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:8000/health || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 3

  redis:
    image: redis:7-alpine
    container_name: trade-alert-redis
    restart: unless-stopped
    
    volumes:
      - ./redis:/data
    
    command: redis-server --appendonly yes --maxmemory 128mb

networks:
  default:
    name: trade-alert-network
EOF

# 创建简单的Dockerfile (如果需要本地构建)
cat > "$PROJECT_DIR/Dockerfile" << 'EOF'
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && \
    cargo build --release --bin trade_alert_rust

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/trade_alert_rust /usr/local/bin/
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static

WORKDIR /app
EXPOSE 8000
CMD ["trade_alert_rust"]
EOF

# 获取内网IP
INTERNAL_IP=$(ip route get 1.1.1.1 | grep -oP 'src \K\S+')

echo -e "${GREEN}✅ 配置文件创建完成${NC}"
echo ""
echo -e "${BLUE}🐳 接下来你可以选择部署方式:${NC}"
echo ""
echo -e "${YELLOW}方案1: 图形化部署 (推荐)${NC}"
echo "1. 打开DSM → Container Manager"
echo "2. 项目 → 新增"
echo "3. 项目名称: trade-alert-rust"
echo "4. 路径: $PROJECT_DIR"
echo "5. 启动项目"
echo ""
echo -e "${YELLOW}方案2: 命令行部署${NC}"
echo "运行命令:"
echo "cd $PROJECT_DIR"
echo "docker-compose up -d"
echo ""

# 显示访问信息
echo -e "${GREEN}🌐 部署完成后访问地址:${NC}"
echo "内网访问: http://$INTERNAL_IP:8000"
echo "本机访问: http://localhost:8000"
echo ""

# 显示下一步操作
echo -e "${BLUE}📋 下一步操作建议:${NC}"
echo ""
echo -e "${YELLOW}1. 配置DDNS (外网访问)${NC}"
echo "   DSM → 控制面板 → 外部访问 → DDNS"
echo ""
echo -e "${YELLOW}2. 配置反向代理 (HTTPS)${NC}"
echo "   DSM → 控制面板 → 应用程序门户 → 反向代理"
echo ""
echo -e "${YELLOW}3. 配置SSL证书${NC}"
echo "   DSM → 控制面板 → 安全性 → 证书"
echo ""
echo -e "${YELLOW}4. 配置路由器端口转发${NC}"
echo "   路由器设置 → 端口转发 → 8000:8000"
echo ""

# 创建管理脚本
echo -e "${YELLOW}🛠️ 创建管理脚本...${NC}"

# 状态检查脚本
cat > "$PROJECT_DIR/status.sh" << 'EOF'
#!/bin/bash
echo "=== TradeAlert 状态检查 ==="
echo ""
echo "🐳 Docker容器状态:"
docker ps --filter name=trade-alert

echo ""
echo "💾 磁盘使用情况:"
du -sh /volume1/docker/trade-alert/*

echo ""
echo "🌐 服务可达性测试:"
curl -s http://localhost:8000/health && echo "✅ 服务正常" || echo "❌ 服务异常"

echo ""
echo "📊 资源使用情况:"
docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.MemPerc}}"
EOF

# 日志查看脚本
cat > "$PROJECT_DIR/logs.sh" << 'EOF'
#!/bin/bash
echo "📝 查看应用日志:"
echo "================"
docker logs -f trade-alert-rust
EOF

# 备份脚本
cat > "$PROJECT_DIR/backup.sh" << 'EOF'
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/volume1/docker/trade-alert/backups"

echo "💾 开始备份数据库..."
docker exec trade-alert-rust sqlite3 /app/data/alerts.db ".backup /app/backups/manual_backup_$DATE.db"

echo "🗜️ 压缩备份文件..."
cd /volume1/docker/trade-alert
tar -czf "$BACKUP_DIR/full_backup_$DATE.tar.gz" data/ config/

echo "🧹 清理30天前的备份..."
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +30 -delete
find "$BACKUP_DIR" -name "*.db" -mtime +30 -delete

echo "✅ 备份完成: backup_$DATE"
EOF

# 设置脚本权限
chmod +x "$PROJECT_DIR"/*.sh

echo -e "${GREEN}✅ 管理脚本创建完成${NC}"
echo ""
echo -e "${BLUE}🎯 管理命令:${NC}"
echo "查看状态: $PROJECT_DIR/status.sh"
echo "查看日志: $PROJECT_DIR/logs.sh"
echo "手动备份: $PROJECT_DIR/backup.sh"
echo ""

# 显示完成信息
echo -e "${GREEN}🎉 Synology部署准备完成！${NC}"
echo ""
echo -e "${YELLOW}📋 最终检查清单:${NC}"
echo "□ 编辑环境变量文件: $ENV_FILE"
echo "□ 在Container Manager中部署项目"
echo "□ 配置DDNS实现外网访问"
echo "□ 设置SSL证书"
echo "□ 配置路由器端口转发"
echo "□ 测试内网访问: http://$INTERNAL_IP:8000"
echo ""

echo -e "${BLUE}💡 提示: 部署完成后可以用DS Finder手机APP随时监控服务状态${NC}" 