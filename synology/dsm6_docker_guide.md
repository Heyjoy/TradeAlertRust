# DSM 6.2.4 Docker 套件部署指南

## 🚨 重要说明
您的 Synology DSM 6.2.4 不支持 Container Manager，需要使用传统的 **Docker** 套件进行部署。

## 📋 系统要求确认
- ✅ DSM 6.2.4-25556
- ✅ 需要安装 Docker 套件（不是 Container Manager）
- ✅ 支持的架构：x86_64

## 🔧 第一步：安装 Docker 套件

### 1. 安装 Docker
1. 打开 **DSM** → **套件中心**
2. 搜索 "**Docker**"（注意：不是 Container Manager）
3. 点击安装 Docker 套件
4. 等待安装完成

### 2. 检查 Docker 版本
安装完成后，Docker 版本可能是较老的版本（如 17.05 或 18.09），这是正常的。

## 📁 第二步：准备部署文件

### 1. 创建项目目录
通过 File Station 创建：
```
/volume1/docker/trade-alert/
├── data/           # 数据库存储
├── logs/           # 日志文件
├── config/         # 配置文件
└── redis/          # Redis数据
```

### 2. 创建 docker-compose.yml
在 `/volume1/docker/trade-alert/` 目录下创建 `docker-compose.yml`：

```yaml
version: '3.3'  # DSM 6.2.4 兼容版本

services:
  trade-alert:
    build:
      context: /volume1/docker/trade-alert-source
      dockerfile: docker/Dockerfile.dsm6
    image: trade-alert-rust:dsm6
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
      
      # 🔧 请修改为你的实际邮件配置
      - EMAIL_SMTP_HOST=smtp.gmail.com
      - EMAIL_SMTP_PORT=587
      - EMAIL_USERNAME=your-email@gmail.com
      - EMAIL_PASSWORD=your-app-password
      - EMAIL_FROM_NAME=TradeAlert股票预警
    
    volumes:
      - /volume1/docker/trade-alert/data:/app/data
      - /volume1/docker/trade-alert/logs:/app/logs
      - /volume1/docker/trade-alert/config:/app/config
    
    # DSM 6.2.4 简化的健康检查
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:8000/health || exit 1"]
      interval: 60s
      timeout: 10s
      retries: 3

  redis:
    image: redis:5-alpine  # DSM 6.2.4 兼容版本
    container_name: trade-alert-redis
    restart: unless-stopped
    
    volumes:
      - /volume1/docker/trade-alert/redis:/data
    
    command: redis-server --appendonly yes --maxmemory 128mb

networks:
  default:
    driver: bridge
```

## 🐳 第三步：创建 DSM 6.2.4 专用 Dockerfile

### 创建 Dockerfile.dsm6
```dockerfile
# DSM 6.2.4 兼容的 Dockerfile
FROM rust:1.60-slim as builder

WORKDIR /app
COPY . .

# 构建配置 - DSM 6.2.4 兼容
ENV CARGO_TERM_COLOR=never
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 编译发布版本
RUN cargo build --release --bin trade_alert_rust

# 运行时镜像 - 使用较老但稳定的基础镜像
FROM debian:buster-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && adduser --disabled-password --gecos '' appuser

# 复制二进制文件
COPY --from=builder /app/target/release/trade_alert_rust /usr/local/bin/
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static

# 创建数据目录
RUN mkdir -p /app/data && chown -R appuser:appuser /app

USER appuser
WORKDIR /app

# 暴露端口
EXPOSE 8000

# 启动命令
CMD ["trade_alert_rust"]
```

## 🚀 第四步：DSM 6.2.4 Docker 图形界面部署

### 方法1：Docker 图形界面（推荐）

1. **打开 Docker 套件**
   - DSM → Docker

2. **导入镜像**
   - 点击 "注册表"
   - 搜索并下载 `redis:5-alpine`

3. **创建容器**
   - 点击 "容器" → "新增"
   - 选择方式：使用 docker-compose

4. **上传 docker-compose 文件**
   - 将准备好的 docker-compose.yml 文件上传
   - 或者直接在界面中创建

### 方法2：SSH 命令行部署

```bash
# SSH 登录到群晖
ssh your-username@your-nas-ip

# 切换到 root 用户
sudo -i

# 进入项目目录
cd /volume1/docker/trade-alert

# 启动服务
docker-compose up -d

# 查看运行状态
docker-compose ps

# 查看日志
docker-compose logs trade-alert
```

## 🔧 第五步：配置邮件设置

### 修改 docker-compose.yml 中的邮件配置
```yaml
environment:
  - EMAIL_USERNAME=你的gmail地址
  - EMAIL_PASSWORD=你的Gmail应用专用密码
  - EMAIL_SMTP_HOST=smtp.gmail.com
  - EMAIL_SMTP_PORT=587
```

### Gmail 配置步骤
1. 开启 Google 账号两步验证
2. 生成应用专用密码：Google账号 → 安全性 → 应用专用密码
3. 使用生成的16位密码作为 EMAIL_PASSWORD

## 🌐 第六步：访问应用

部署完成后访问：`http://群晖IP:8000`

## ❗ DSM 6.2.4 特有注意事项

### 1. Docker 版本限制
- DSM 6.2.4 的 Docker 版本较老
- 某些新的 Docker 功能可能不支持
- 建议使用稳定的基础镜像

### 2. compose 文件版本
- 使用 `version: '3.3'` 或更低版本
- 避免使用过新的 compose 语法

### 3. 网络配置
- DSM 6.2.4 的网络功能相对简单
- 建议使用默认 bridge 网络

### 4. 资源监控
- DSM 6.2.4 的容器监控功能有限
- 建议通过 SSH 使用 `docker stats` 命令监控

## 🔍 故障排除

### 容器启动失败
```bash
# 查看详细日志
docker logs trade-alert-rust

# 检查镜像
docker images

# 重新构建镜像
docker-compose build --no-cache
```

### 端口访问问题
```bash
# 检查端口占用
netstat -tulpn | grep 8000

# 检查防火墙
# DSM → 控制面板 → 安全性 → 防火墙
```

## 🎯 成功部署检查清单

- [ ] Docker 套件已安装
- [ ] 项目目录已创建
- [ ] docker-compose.yml 已配置
- [ ] 邮件配置已修改
- [ ] 容器成功启动
- [ ] 能够访问 http://群晖IP:8000
- [ ] 邮件发送功能正常

## 📈 后续优化建议

1. **定期备份**
   - 设置数据库定期备份
   - 备份 docker-compose.yml 配置

2. **监控设置**
   - 使用 DSM 任务计划监控容器状态
   - 设置邮件通知

3. **升级考虑**
   - 考虑升级到 DSM 7.x 以获得更好的容器管理功能
   - 升级前确保数据已备份

这个方案专门针对您的 DSM 6.2.4 环境优化，应该能够成功部署 TradeAlert 系统！ 