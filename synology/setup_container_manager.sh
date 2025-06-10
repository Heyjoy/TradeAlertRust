#!/bin/bash

# 🏠 Synology Container Manager 一键准备脚本
# 使用方法: 将项目代码上传到群晖后运行此脚本

set -e

echo "🏠 准备 Synology Container Manager 部署"
echo "========================================"

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# 检测环境
if [ ! -d "/volume1" ]; then
    echo -e "${RED}❌ 未检测到Synology环境！${NC}"
    echo "请在群晖NAS上运行此脚本"
    exit 1
fi

echo -e "${GREEN}✅ 检测到Synology环境${NC}"

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

# 复制源代码
sudo cp -r "$SOURCE_DIR" "/volume1/docker/trade-alert-source"
sudo chown -R $(whoami):users "/volume1/docker/trade-alert-source"

# 复制docker-compose文件
cp "$SOURCE_DIR/synology/simple-docker-compose.yml" "$PROJECT_DIR/docker-compose.yml"

echo -e "${GREEN}✅ 文件复制完成${NC}"

# 获取内网IP
INTERNAL_IP=$(ip route get 1.1.1.1 | grep -oP 'src \K\S+' 2>/dev/null || hostname -I | awk '{print $1}')

echo ""
echo -e "${BLUE}🎉 准备工作完成！${NC}"
echo ""
echo -e "${YELLOW}📋 接下来请按照以下步骤操作:${NC}"
echo ""
echo "1️⃣  打开 DSM → Container Manager"
echo "2️⃣  点击 '项目' → '新增'"
echo "3️⃣  项目设置:"
echo "    项目名称: trade-alert-rust"
echo "    路径: $PROJECT_DIR"
echo "    数据源: 选择'设置路径'"
echo ""
echo "4️⃣  ⚠️  重要: 修改docker-compose.yml中的邮件配置"
echo "    文件位置: $PROJECT_DIR/docker-compose.yml"
echo "    需要修改的变量:"
echo "    - EMAIL_USERNAME=你的邮箱@gmail.com"
echo "    - EMAIL_PASSWORD=你的应用专用密码"
echo ""
echo "5️⃣  点击'下一步'启动项目"
echo ""
echo -e "${GREEN}🌐 部署完成后访问地址:${NC}"
echo "http://$INTERNAL_IP:8000"
echo ""
echo -e "${YELLOW}💡 邮件配置提示:${NC}"
echo "- Gmail用户需要开启两步验证"
echo "- 使用应用专用密码，不是登录密码"
echo "- 应用专用密码获取: Google账号 → 安全性 → 应用专用密码"

# 显示项目文件路径
echo ""
echo -e "${BLUE}📂 重要文件位置:${NC}"
echo "项目目录: $PROJECT_DIR"
echo "源代码: /volume1/docker/trade-alert-source"
echo "配置文件: $PROJECT_DIR/docker-compose.yml" 