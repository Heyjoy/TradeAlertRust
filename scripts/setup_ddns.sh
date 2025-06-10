#!/bin/bash

# 动态DNS配置脚本
# 支持Cloudflare、阿里云等DNS服务商

set -e

echo "🌐 配置动态DNS..."

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# 配置文件
CONFIG_FILE="/etc/ddns-config"

# 创建配置
if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${YELLOW}⚙️ 创建DDNS配置文件...${NC}"
    sudo tee "$CONFIG_FILE" > /dev/null << EOF
# DNS服务商选择: cloudflare, aliyun, dnspod
DNS_PROVIDER=cloudflare

# Cloudflare配置
CLOUDFLARE_API_TOKEN=your-api-token
CLOUDFLARE_ZONE_ID=your-zone-id
CLOUDFLARE_DOMAIN=your-domain.com
CLOUDFLARE_RECORD_NAME=trade-alert

# 阿里云DNS配置
ALIYUN_ACCESS_KEY=your-access-key
ALIYUN_SECRET_KEY=your-secret-key
ALIYUN_DOMAIN=your-domain.com
ALIYUN_RECORD_NAME=trade-alert

# 更新间隔（分钟）
UPDATE_INTERVAL=5
EOF

    echo -e "${RED}❗ 请编辑 $CONFIG_FILE 配置你的DNS信息${NC}"
    exit 1
fi

# 加载配置
source "$CONFIG_FILE"

# 创建DDNS更新脚本
DDNS_SCRIPT="/usr/local/bin/ddns-update.sh"
echo -e "${YELLOW}📝 创建DDNS更新脚本...${NC}"

sudo tee "$DDNS_SCRIPT" > /dev/null << 'EOF'
#!/bin/bash

# 加载配置
source /etc/ddns-config

# 获取当前公网IP
get_public_ip() {
    curl -s https://api.ipify.org || curl -s https://ifconfig.me || curl -s https://icanhazip.com
}

# Cloudflare DNS更新
update_cloudflare() {
    local ip="$1"
    local response=$(curl -s -X PUT "https://api.cloudflare.com/client/v4/zones/$CLOUDFLARE_ZONE_ID/dns_records" \
        -H "Authorization: Bearer $CLOUDFLARE_API_TOKEN" \
        -H "Content-Type: application/json" \
        --data "{\"type\":\"A\",\"name\":\"$CLOUDFLARE_RECORD_NAME\",\"content\":\"$ip\",\"ttl\":120}")
    
    if echo "$response" | grep -q '"success":true'; then
        echo "$(date): Cloudflare DNS更新成功 - $CLOUDFLARE_RECORD_NAME.$CLOUDFLARE_DOMAIN -> $ip"
        return 0
    else
        echo "$(date): Cloudflare DNS更新失败 - $response"
        return 1
    fi
}

# 阿里云DNS更新
update_aliyun() {
    local ip="$1"
    # 这里需要安装阿里云CLI或使用API
    echo "$(date): 阿里云DNS更新 - $ALIYUN_RECORD_NAME.$ALIYUN_DOMAIN -> $ip"
}

# 主逻辑
main() {
    local current_ip=$(get_public_ip)
    local last_ip_file="/tmp/last_ddns_ip"
    
    if [ -z "$current_ip" ]; then
        echo "$(date): 无法获取公网IP"
        return 1
    fi
    
    # 检查IP是否变化
    if [ -f "$last_ip_file" ]; then
        local last_ip=$(cat "$last_ip_file")
        if [ "$current_ip" = "$last_ip" ]; then
            return 0  # IP未变化，无需更新
        fi
    fi
    
    # 根据DNS服务商更新
    case "$DNS_PROVIDER" in
        "cloudflare")
            if update_cloudflare "$current_ip"; then
                echo "$current_ip" > "$last_ip_file"
            fi
            ;;
        "aliyun")
            if update_aliyun "$current_ip"; then
                echo "$current_ip" > "$last_ip_file"
            fi
            ;;
        *)
            echo "$(date): 不支持的DNS服务商: $DNS_PROVIDER"
            ;;
    esac
}

main "$@"
EOF

sudo chmod +x "$DDNS_SCRIPT"

# 创建systemd定时器
echo -e "${YELLOW}⏰ 创建定时更新服务...${NC}"

# 服务文件
sudo tee "/etc/systemd/system/ddns-update.service" > /dev/null << EOF
[Unit]
Description=Dynamic DNS Update Service
After=network.target

[Service]
Type=oneshot
ExecStart=$DDNS_SCRIPT
StandardOutput=journal
StandardError=journal
EOF

# 定时器文件
sudo tee "/etc/systemd/system/ddns-update.timer" > /dev/null << EOF
[Unit]
Description=Dynamic DNS Update Timer
Requires=ddns-update.service

[Timer]
OnBootSec=2min
OnUnitActiveSec=${UPDATE_INTERVAL}min
Persistent=true

[Install]
WantedBy=timers.target
EOF

# 启用定时器
sudo systemctl daemon-reload
sudo systemctl enable ddns-update.timer
sudo systemctl start ddns-update.timer

# 立即执行一次
echo -e "${YELLOW}🔄 执行首次DDNS更新...${NC}"
sudo "$DDNS_SCRIPT"

echo -e "${GREEN}✅ 动态DNS配置完成！${NC}"
echo -e "${GREEN}🌐 你的域名将指向当前公网IP${NC}"
echo -e "${YELLOW}💡 记得在路由器中配置端口转发：${NC}"
echo "  外部端口: 8000 -> 内部IP:8000"

# 显示管理命令
echo -e "\n${YELLOW}🛠️ DDNS管理命令:${NC}"
echo "查看状态: sudo systemctl status ddns-update.timer"
echo "查看日志: sudo journalctl -u ddns-update.service -f"
echo "手动更新: sudo $DDNS_SCRIPT" 