# Synology Container Manager 图形化部署指南

## 🏠 准备工作

### 1. 检查Container Manager
1. 打开 **DSM** (Synology管理界面)
2. 进入 **套件中心**
3. 搜索 "**Container Manager**"
4. 如果未安装，点击安装并等待完成

### 2. 创建项目目录
通过File Station创建以下目录结构：
```
/volume1/docker/trade-alert/
├── data/           # 数据库存储
├── logs/           # 日志文件
├── config/         # 配置文件
├── backups/        # 备份文件
└── redis/          # Redis数据
```

## 📦 第一步：创建项目

### 1. 打开Container Manager
1. 在DSM中点击 **Container Manager**
2. 点击左侧菜单 **"项目"**
3. 点击 **"新增"** 按钮

### 2. 配置项目基本信息
- **项目名称**: `trade-alert-rust`
- **路径**: `/volume1/docker/trade-alert`
- **数据源**: 选择 **"创建docker-compose.yml"**

## ⚙️ 第二步：配置docker-compose.yml

在编辑器中输入以下内容：

```yaml
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
      - RUST_LOG=info
      - TZ=Asia/Shanghai
      
      # 邮件配置 - 请修改为你的实际配置
      - EMAIL_SMTP_HOST=smtp.gmail.com
      - EMAIL_SMTP_PORT=587
      - EMAIL_USERNAME=your-email@gmail.com
      - EMAIL_PASSWORD=your-app-password
      - EMAIL_FROM_NAME=TradeAlert
    
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
```

## 🔧 第三步：配置环境变量

### 重要：修改邮件配置
在上面的docker-compose.yml中，必须修改以下参数：

```yaml
- EMAIL_SMTP_HOST=smtp.gmail.com          # 你的SMTP服务器
- EMAIL_SMTP_PORT=587                     # SMTP端口
- EMAIL_USERNAME=your-email@gmail.com     # 你的邮箱地址
- EMAIL_PASSWORD=your-app-password        # 你的邮箱应用密码
- EMAIL_FROM_NAME=TradeAlert              # 发件人名称
```

### Gmail配置示例：
1. **EMAIL_USERNAME**: 你的Gmail地址
2. **EMAIL_PASSWORD**: Gmail应用专用密码 (不是登录密码)
   - 设置方法：Google账号 → 安全性 → 应用专用密码

## 🚀 第四步：部署项目

### 1. 构建并启动
1. 点击 **"下一步"**
2. 检查配置无误后，点击 **"完成"**
3. Container Manager会自动拉取镜像并启动服务

### 2. 检查运行状态
1. 在 **"项目"** 页面找到 `trade-alert-rust`
2. 查看状态应该为 **"正在运行"**
3. 点击项目名称查看详细信息

## 🌐 第五步：访问应用

### 1. 获取访问地址
- **内网访问**: `http://群晖IP:8000`
- **例如**: `http://192.168.1.100:8000`

### 2. 测试功能
1. 打开浏览器访问上述地址
2. 输入股票代码 (如 AAPL, TSLA)
3. 设置价格预警条件
4. 测试邮件发送功能

## 🔍 第六步：监控和管理

### 1. 查看日志
1. Container Manager → 项目 → trade-alert-rust
2. 点击 **trade-alert-rust** 容器
3. 查看 **"日志"** 标签页

### 2. 管理容器
- **启动/停止**: 点击对应按钮
- **重新启动**: 右键菜单 → 重新启动
- **更新镜像**: 右键菜单 → 停止 → 重新构建

## 🌍 第七步：外网访问 (可选)

### 1. 配置DDNS
1. DSM → 控制面板 → 外部访问 → DDNS
2. 申请免费域名 (如 synology.me)

### 2. 配置反向代理
1. DSM → 控制面板 → 应用程序门户 → 反向代理
2. 新增反向代理规则：
   - **来源**: 端口 80/443
   - **目标**: localhost:8000

### 3. 配置路由器端口转发
- 转发端口 8000 到群晖内网IP

## ❗ 故障排除

### 容器无法启动
1. 检查邮件配置是否正确
2. 确认目录权限 (chmod 755)
3. 查看容器日志错误信息

### 无法访问应用
1. 确认防火墙设置
2. 检查端口 8000 是否被占用
3. 验证群晖IP地址

### 邮件发送失败
1. 检查EMAIL_PASSWORD是否为应用专用密码
2. 确认Gmail两步验证已开启
3. 测试SMTP服务器连接

## 🎉 完成

恭喜！你已经成功在Synology NAS上部署了TradeAlert交易预警系统。

**下一步建议**：
1. 设置外网访问以便随时查看
2. 配置定期数据备份
3. 邀请朋友测试系统功能 