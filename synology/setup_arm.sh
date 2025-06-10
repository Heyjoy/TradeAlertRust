#!/bin/bash

# 🏠 ARM 架构 Synology NAS 自动化部署脚本
# 支持 armv7l 和 aarch64 架构

set -e

echo "🏠 ARM 架构 Synology 部署 TradeAlert"
echo "====================================="

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

# 检测CPU架构
ARCH=$(uname -m)
echo -e "${BLUE}📋 CPU架构: $ARCH${NC}"

case $ARCH in
    armv7l)
        echo -e "${YELLOW}📱 检测到32位ARM架构${NC}"
        ARM_TYPE="armv7l"
        ;;
    aarch64)
        echo -e "${YELLOW}📱 检测到64位ARM架构${NC}"
        ARM_TYPE="aarch64"
        ;;
    *)
        echo -e "${RED}❌ 不支持的架构: $ARCH${NC}"
        echo -e "${YELLOW}💡 此脚本仅支持 ARM 架构${NC}"
        exit 1
        ;;
esac

# 配置变量
PROJECT_DIR="/volume1/trade-alert"
SOURCE_DIR="$(pwd)"

echo ""
echo -e "${BLUE}🔧 部署方案选择${NC}"
echo "1) 手动安装 Docker + 容器部署 (功能完整但有风险)"
echo "2) 原生 Rust 应用部署 (安全推荐)"
echo "3) 退出"
echo ""
read -p "请选择部署方案 (1-3): " choice

case $choice in
    1)
        echo -e "${YELLOW}🐳 选择了 Docker 容器部署${NC}"
        DEPLOY_METHOD="docker"
        ;;
    2)
        echo -e "${YELLOW}🦀 选择了原生 Rust 部署${NC}"
        DEPLOY_METHOD="native"
        ;;
    3)
        echo "退出部署"
        exit 0
        ;;
    *)
        echo -e "${RED}❌ 无效选择${NC}"
        exit 1
        ;;
esac

# 创建项目目录
echo -e "${YELLOW}📁 创建项目目录...${NC}"
sudo mkdir -p "$PROJECT_DIR"/{source,data,logs,config,redis}
sudo chmod -R 755 "$PROJECT_DIR"
sudo chown -R $(whoami):users "$PROJECT_DIR"

# 复制源代码
echo -e "${YELLOW}📋 复制项目文件...${NC}"
cp -r "$SOURCE_DIR"/* "$PROJECT_DIR/source/"

if [ "$DEPLOY_METHOD" = "docker" ]; then
    # Docker 部署方案
    echo ""
    echo -e "${RED}⚠️  Docker 安装风险警告 ⚠️${NC}"
    echo -e "${RED}此方法完全不受 Synology 官方支持！${NC}"
    echo -e "${RED}可能会损坏您的 NAS 系统！${NC}"
    echo -e "${YELLOW}请确保重要数据已备份！${NC}"
    echo ""
    read -p "您确定要继续吗？(输入 YES 继续): " confirm
    
    if [ "$confirm" != "YES" ]; then
        echo "取消安装"
        exit 0
    fi
    
    echo -e "${YELLOW}🐳 开始安装 Docker...${NC}"
    
    # 检查是否已安装 Docker
    if command -v docker &> /dev/null; then
        echo -e "${GREEN}✅ Docker 已安装${NC}"
    else
        echo -e "${YELLOW}📦 安装 Docker...${NC}"
        
        # 根据架构选择安装方法
        if [ "$ARM_TYPE" = "armv7l" ]; then
            # 32位 ARM
            curl https://gist.githubusercontent.com/ta264/2b7fb6e6466b109b9bf9b0a1d91ebedc/raw/b76a28d25d0abd0d27a0c9afaefa0d499eb87d3d/get-docker.sh | sudo sh
        else
            # 64位 ARM
            wget https://github.com/ypkdani00/docker-on-synology-arm64/raw/main/install.sh
            chmod +x install.sh
            sudo ./install.sh
            rm install.sh
        fi
        
        # 配置 Docker
        sudo mkdir -p /volume1/@docker
        sudo mkdir -p /etc/docker
        
        cat | sudo tee /etc/docker/daemon.json > /dev/null << 'EOF'
{
  "storage-driver": "vfs",
  "iptables": false,
  "data-root": "/volume1/@docker"
}
EOF
        
        echo -e "${GREEN}✅ Docker 安装完成${NC}"
    fi
    
    # 创建 ARM 专用的 docker-compose 文件
    cat > "$PROJECT_DIR/docker-compose.yml" << 'EOF'
version: '3.3'

services:
  trade-alert:
    image: rust:1.60-slim
    container_name: trade-alert-rust
    restart: unless-stopped
    network_mode: host
    
    environment:
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
      - DATABASE_URL=sqlite:///app/data/alerts.db
      - RUST_LOG=info
      - TZ=Asia/Shanghai
      
      # 邮件配置 - 请修改
      - EMAIL_SMTP_HOST=smtp.gmail.com
      - EMAIL_SMTP_PORT=587
      - EMAIL_USERNAME=your-email@gmail.com
      - EMAIL_PASSWORD=your-app-password
      - EMAIL_FROM_NAME=TradeAlert
    
    volumes:
      - ./source:/app/source
      - ./data:/app/data
      - ./logs:/app/logs
    
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
    image: redis:5-alpine
    container_name: trade-alert-redis
    restart: unless-stopped
    network_mode: host
    
    volumes:
      - ./redis:/data
EOF
    
    echo -e "${GREEN}✅ Docker 配置完成${NC}"
    
elif [ "$DEPLOY_METHOD" = "native" ]; then
    # 原生 Rust 部署方案
    echo -e "${YELLOW}🦀 原生 Rust 部署${NC}"
    
    # 检查 Rust 是否已安装
    if command -v cargo &> /dev/null; then
        echo -e "${GREEN}✅ Rust 已安装${NC}"
    else
        echo -e "${YELLOW}📦 安装 Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    fi
    
    # 检查系统依赖
    echo -e "${YELLOW}🔧 检查系统依赖...${NC}"
    
    # 尝试不同的包管理器
    if command -v opkg &> /dev/null; then
        echo -e "${GREEN}✅ 找到 opkg 包管理器${NC}"
        sudo opkg update
        sudo opkg install gcc || echo -e "${YELLOW}⚠️ gcc 安装失败，可能需要手动处理${NC}"
        sudo opkg install openssl-dev || echo -e "${YELLOW}⚠️ openssl-dev 安装失败${NC}"
        sudo opkg install pkg-config || echo -e "${YELLOW}⚠️ pkg-config 安装失败${NC}"
    elif command -v ipkg &> /dev/null; then
        echo -e "${GREEN}✅ 找到 ipkg 包管理器${NC}"
        sudo ipkg update
        sudo ipkg install gcc-opt
        sudo ipkg install openssl-dev
    else
        echo -e "${YELLOW}⚠️ 未找到包管理器，可能需要手动安装依赖${NC}"
        echo -e "${YELLOW}💡 建议安装 Entware 获得更好的包管理支持${NC}"
    fi
    
    # 创建启动脚本
    cat > "$PROJECT_DIR/start.sh" << 'EOF'
#!/bin/bash

export SERVER_HOST=0.0.0.0
export SERVER_PORT=8000
export DATABASE_URL=sqlite:///volume1/trade-alert/data/alerts.db
export RUST_LOG=info
export TZ=Asia/Shanghai

# 邮件配置 - 请修改为你的实际配置
export EMAIL_SMTP_HOST=smtp.gmail.com
export EMAIL_SMTP_PORT=587
export EMAIL_USERNAME=your-email@gmail.com
export EMAIL_PASSWORD=your-app-password
export EMAIL_FROM_NAME=TradeAlert

# 切换到项目目录
cd /volume1/trade-alert/source

# 启动应用
./target/release/trade_alert_rust
EOF
    
    chmod +x "$PROJECT_DIR/start.sh"
    
    echo -e "${GREEN}✅ 原生部署配置完成${NC}"
fi

# 获取内网IP
INTERNAL_IP=$(ip route get 1.1.1.1 | grep -oP 'src \K\S+' 2>/dev/null || hostname -I | awk '{print $1}')

echo ""
echo -e "${BLUE}🎉 ARM 架构部署准备完成！${NC}"
echo ""
echo -e "${RED}⚠️ 重要：请先修改邮件配置！${NC}"
echo ""

if [ "$DEPLOY_METHOD" = "docker" ]; then
    echo -e "${YELLOW}📋 Docker 部署后续步骤:${NC}"
    echo ""
    echo "1️⃣ 修改邮件配置："
    echo "   编辑文件: $PROJECT_DIR/docker-compose.yml"
    echo "   修改 EMAIL_USERNAME 和 EMAIL_PASSWORD"
    echo ""
    echo "2️⃣ 启动容器："
    echo "   cd $PROJECT_DIR"
    echo "   sudo dockerd &"
    echo "   docker-compose up -d"
    echo ""
    echo "3️⃣ 设置开机自启："
    echo "   DSM → 控制面板 → 任务计划"
    echo "   新增 → 触发的任务 → 用户定义的脚本"
    echo "   任务：Docker启动"
    echo "   用户：root"
    echo "   事件：开机"
    echo "   运行命令：dockerd &"
    
elif [ "$DEPLOY_METHOD" = "native" ]; then
    echo -e "${YELLOW}📋 原生部署后续步骤:${NC}"
    echo ""
    echo "1️⃣ 修改邮件配置："
    echo "   编辑文件: $PROJECT_DIR/start.sh"
    echo "   修改 EMAIL_USERNAME 和 EMAIL_PASSWORD"
    echo ""
    echo "2️⃣ 编译应用："
    echo "   cd $PROJECT_DIR/source"
    echo "   source ~/.cargo/env"
    echo "   cargo build --release --bin trade_alert_rust"
    echo ""
    echo "3️⃣ 启动应用："
    echo "   $PROJECT_DIR/start.sh"
    echo ""
    echo "4️⃣ 设置开机自启："
    echo "   DSM → 控制面板 → 任务计划"
    echo "   新增 → 触发的任务 → 用户定义的脚本"
    echo "   任务：TradeAlert启动"
    echo "   用户：你的用户名"
    echo "   事件：开机"
    echo "   运行命令：$PROJECT_DIR/start.sh"
fi

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
echo "源代码: $PROJECT_DIR/source"
echo "数据目录: $PROJECT_DIR/data"
echo "日志目录: $PROJECT_DIR/logs"

if [ "$DEPLOY_METHOD" = "docker" ]; then
    echo "配置文件: $PROJECT_DIR/docker-compose.yml"
else
    echo "启动脚本: $PROJECT_DIR/start.sh"
fi 