#!/bin/bash

# 🏠 Synology DSM 6.2.4 Docker 套件部署脚本
# 专门适配 DSM 6.2.4-25556

set -e

echo "🏠 Synology DSM 6.2.4 Docker 部署"
echo "=================================="

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# 检测环境
if [ ! -d "/volume1" ]; then
    echo -e "${RED}❌ 未检测到Synology环境！${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 检测到Synology环境${NC}"

# 检查DSM版本
DSM_VERSION=$(cat /etc.defaults/VERSION | grep productversion | cut -d'"' -f4)
echo -e "${BLUE}📋 DSM 版本: $DSM_VERSION${NC}"

if [[ ! "$DSM_VERSION" =~ ^6\.2 ]]; then
    echo -e "${YELLOW}⚠️  当前脚本专为 DSM 6.2.x 设计${NC}"
    echo -e "${YELLOW}💡 如果是 DSM 7.x，请使用 Container Manager 部署方案${NC}"
fi

# 检查Docker套件
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker 套件未安装！${NC}"
    echo -e "${YELLOW}📦 请先安装 Docker 套件：${NC}"
    echo "1. DSM → 套件中心"
    echo "2. 搜索 'Docker'"
    echo "3. 安装 Docker 套件"
    exit 1
fi

echo -e "${GREEN}✅ Docker 套件检查通过${NC}"

# 检查docker-compose
if ! command -v docker-compose &> /dev/null; then
    echo -e "${YELLOW}⚠️  docker-compose 未找到，尝试安装...${NC}"
    
    # DSM 6.2.4 可能需要手动安装 docker-compose
    if command -v pip3 &> /dev/null; then
        sudo pip3 install docker-compose
    else
        echo -e "${RED}❌ 需要手动安装 docker-compose${NC}"
        echo "请通过 SSH 运行: sudo pip3 install docker-compose"
        exit 1
    fi
fi

echo -e "${GREEN}✅ docker-compose 检查通过${NC}"

# 配置变量
PROJECT_DIR="/volume1/docker/trade-alert"
SOURCE_DIR="$(pwd)"

echo -e "${YELLOW}📁 创建项目目录结构...${NC}"

# 创建目录
sudo mkdir -p "$PROJECT_DIR"/{data,logs,config,redis}
sudo chmod -R 755 "$PROJECT_DIR"
sudo chown -R $(whoami):users "$PROJECT_DIR"

echo -e "${GREEN}✅ 目录创建完成${NC}"

# 复制项目文件
echo -e "${YELLOW}📋 复制项目文件...${NC}"

# 复制源代码到标准位置
sudo cp -r "$SOURCE_DIR" "/volume1/docker/trade-alert-source"
sudo chown -R $(whoami):users "/volume1/docker/trade-alert-source"

# 复制 DSM 6.2.4 专用配置
cp "$SOURCE_DIR/synology/docker-compose.dsm6.yml" "$PROJECT_DIR/docker-compose.yml"

echo -e "${GREEN}✅ 文件复制完成${NC}"

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
EMAIL_FROM_NAME=TradeAlert股票预警

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
fi

# 获取内网IP
INTERNAL_IP=$(ip route get 1.1.1.1 | grep -oP 'src \K\S+' 2>/dev/null || hostname -I | awk '{print $1}')

echo ""
echo -e "${BLUE}🎉 DSM 6.2.4 部署准备完成！${NC}"
echo ""
echo -e "${YELLOW}📋 接下来请按照以下步骤操作:${NC}"
echo ""
echo -e "${GREEN}方法1: Docker 图形界面部署 (推荐)${NC}"
echo "1️⃣  打开 DSM → Docker"
echo "2️⃣  点击 '容器' → '新增'"
echo "3️⃣  选择 'docker-compose' 方式"
echo "4️⃣  上传配置文件: $PROJECT_DIR/docker-compose.yml"
echo "5️⃣  修改邮件配置参数"
echo "6️⃣  启动容器"
echo ""
echo -e "${GREEN}方法2: SSH 命令行部署${NC}"
echo "cd $PROJECT_DIR"
echo "# 编辑 docker-compose.yml 中的邮件配置"
echo "nano docker-compose.yml"
echo "# 启动服务"
echo "docker-compose up -d"
echo ""
echo -e "${RED}⚠️  重要: 必须先修改邮件配置！${NC}"
echo "文件位置: $PROJECT_DIR/docker-compose.yml"
echo "需要修改的变量:"
echo "- EMAIL_USERNAME=你的邮箱@gmail.com"
echo "- EMAIL_PASSWORD=你的应用专用密码"
echo ""
echo -e "${GREEN}🌐 部署完成后访问地址:${NC}"
echo "http://$INTERNAL_IP:8000"
echo ""
echo -e "${YELLOW}💡 Gmail 配置提示:${NC}"
echo "- 开启 Google 账号两步验证"
echo "- 生成应用专用密码: Google账号 → 安全性 → 应用专用密码"
echo "- 使用16位应用专用密码，不是登录密码"
echo ""
echo -e "${BLUE}📂 重要文件位置:${NC}"
echo "项目目录: $PROJECT_DIR"
echo "源代码: /volume1/docker/trade-alert-source"
echo "配置文件: $PROJECT_DIR/docker-compose.yml"
echo "环境变量: $PROJECT_DIR/.env" 