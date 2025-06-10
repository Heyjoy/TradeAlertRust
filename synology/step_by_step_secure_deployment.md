# 🛡️ ARM Synology 安全原生部署 - 一步步指南

## 📋 部署总览

我们将通过以下步骤安全部署 TradeAlert：
1. 配置 Synology 权限设置
2. 在 Windows 上交叉编译 ARM 版本
3. 安全上传到 NAS
4. 配置安全运行环境
5. 设置自动启动和监控

## 🔧 第一部分：Synology 权限配置

### 步骤1：开启 SSH 访问
1. **登录 DSM**
   - 打开浏览器访问：`http://群晖IP:5000`
   - 输入管理员账号密码

2. **开启 SSH 服务**
   ```
   DSM → 控制面板 → 终端机和 SNMP → 终端机
   勾选：☑️ 启动 SSH 功能
   端口：22 (默认)
   点击：应用
   ```

3. **配置防火墙（重要安全措施）**
   ```
   DSM → 控制面板 → 安全性 → 防火墙 → 编辑规则
   新增规则：
   - 端口：SSH (22)
   - 源IP：您的内网IP段 (如 192.168.1.0/24)
   - 动作：允许
   ```

### 步骤2：创建专用用户（推荐）
1. **创建应用专用用户**
   ```
   DSM → 控制面板 → 用户账号 → 新增
   用户名：tradealert
   密码：（设置强密码）
   描述：TradeAlert应用专用账户
   
   群组：users
   权限：只给必要的文件夹访问权限
   ```

2. **设置用户权限**
   - 勾选需要的共享文件夹权限
   - **不要**给予管理员权限
   - 可以访问 SSH：☑️

### 步骤3：创建应用目录
1. **通过 File Station 创建目录**
   ```
   File Station → volume1 → 新增文件夹
   文件夹名：apps
   
   apps → 新增文件夹
   文件夹名：trade-alert
   ```

2. **设置目录权限**
   ```
   右键 trade-alert → 属性 → 权限
   所有者：tradealert
   群组：users
   权限：读取/写入
   ```

## 💻 第二部分：Windows 交叉编译

### 步骤4：安装 Rust 交叉编译支持

在您的 Windows 开发机器上执行：

1. **检查当前 Rust 安装**
   ```powershell
   rustc --version
   cargo --version
   ```

2. **添加 ARM 编译目标**
   ```powershell
   # 根据您的 NAS 架构选择：
   
   # 如果是 32位 ARM (armv7l)
   rustup target add armv7-unknown-linux-gnueabihf
   
   # 如果是 64位 ARM (aarch64)  
   rustup target add aarch64-unknown-linux-gnu
   ```

3. **安装交叉编译器** 
   
   下载并安装以下工具之一：
   
   **选项A：使用 cargo-cross（推荐）**
   ```powershell
   cargo install cross
   ```
   
   **选项B：手动配置链接器**
   - 下载 ARM GCC 工具链
   - 配置 .cargo/config.toml

### 步骤5：编译 ARM 版本

1. **进入项目目录**
   ```powershell
   cd F:\OneDrive\01Project\TradeAlertRust
   ```

2. **检查 NAS 架构**
   
   先通过 SSH 连接到 NAS：
   ```bash
   ssh tradealert@群晖IP
   uname -m
   ```
   
   记下输出结果（armv7l 或 aarch64）

3. **编译对应版本**
   
   **如果是 32位 ARM (armv7l):**
   ```powershell
   # 使用 cross 工具
   cross build --release --target armv7-unknown-linux-gnueabihf --bin trade_alert_rust
   ```
   
   **如果是 64位 ARM (aarch64):**
   ```powershell
   # 使用 cross 工具  
   cross build --release --target aarch64-unknown-linux-gnu --bin trade_alert_rust
   ```

4. **验证编译结果**
   ```powershell
   # 32位 ARM
   ls target\armv7-unknown-linux-gnueabihf\release\trade_alert_rust.exe
   
   # 64位 ARM
   ls target\aarch64-unknown-linux-gnu\release\trade_alert_rust.exe
   ```

## 📤 第三部分：安全上传到 NAS

### 步骤6：上传编译好的程序

1. **使用 SCP 上传**
   
   **32位 ARM:**
   ```powershell
   scp target\armv7-unknown-linux-gnueabihf\release\trade_alert_rust tradealert@群晖IP:/volume1/apps/trade-alert/
   ```
   
   **64位 ARM:**
   ```powershell
   scp target\aarch64-unknown-linux-gnu\release\trade_alert_rust tradealert@群晖IP:/volume1/apps/trade-alert/
   ```

2. **上传静态文件**
   ```powershell
   scp -r templates tradealert@群晖IP:/volume1/apps/trade-alert/
   scp -r static tradealert@群晖IP:/volume1/apps/trade-alert/
   ```

## 🔒 第四部分：NAS 安全配置

### 步骤7：创建安全运行环境

SSH 连接到 NAS：
```bash
ssh tradealert@群晖IP
```

1. **创建目录结构**
   ```bash
   cd /volume1/apps/trade-alert
   mkdir -p {data,logs,config,backup}
   ```

2. **设置文件权限**
   ```bash
   chmod +x trade_alert_rust
   chmod 700 data logs backup
   chmod 755 config
   ```

3. **创建配置文件**
   ```bash
   cat > config/app.env << 'EOF'
   # 网络安全配置
   SERVER_HOST=127.0.0.1
   SERVER_PORT=8000
   
   # 数据库配置
   DATABASE_URL=sqlite:///volume1/apps/trade-alert/data/alerts.db
   
   # 日志配置
   RUST_LOG=info
   
   # 邮件配置 - 请修改为您的实际配置
   EMAIL_SMTP_HOST=smtp.gmail.com
   EMAIL_SMTP_PORT=587
   EMAIL_USERNAME=your-email@gmail.com
   EMAIL_PASSWORD=your-gmail-app-password
   EMAIL_FROM_NAME=TradeAlert股票预警
   
   # 安全配置
   TZ=Asia/Shanghai
   EOF
   
   chmod 600 config/app.env
   ```

4. **创建启动脚本**
   ```bash
   cat > start.sh << 'EOF'
   #!/bin/bash
   
   # 安全检查
   if [ "$(id -u)" = "0" ]; then
       echo "错误：不应该以root用户运行"
       exit 1
   fi
   
   # 设置工作目录
   cd /volume1/apps/trade-alert
   
   # 加载环境变量
   set -a
   source config/app.env
   set +a
   
   # 启动应用
   echo "$(date): 启动 TradeAlert" >> logs/app.log
   ./trade_alert_rust 2>&1 | tee -a logs/app.log
   EOF
   
   chmod +x start.sh
   ```

### 步骤8：测试运行

1. **首次运行测试**
   ```bash
   ./start.sh
   ```
   
   如果看到类似输出表示成功：
   ```
   启动 TradeAlert
   [INFO] Starting server on 127.0.0.1:8000
   ```

2. **测试访问**
   
   在 NAS 内网访问：
   ```
   http://群晖IP:8000
   ```

3. **停止测试**
   ```bash
   Ctrl+C
   ```

## 🚀 第五部分：自动启动配置

### 步骤9：配置开机自启

1. **返回 DSM 界面**
   ```
   DSM → 控制面板 → 任务计划 → 新增
   ```

2. **创建开机任务**
   ```
   任务类型：触发的任务 → 用户定义的脚本
   
   常规设置：
   - 任务名称：TradeAlert启动
   - 用户：tradealert
   - 事件：开机
   
   任务设置：
   - 运行命令：/volume1/apps/trade-alert/start.sh
   ```

3. **创建监控任务**
   ```
   任务类型：计划的任务 → 用户定义的脚本
   
   常规设置：
   - 任务名称：TradeAlert监控
   - 用户：tradealert
   
   计划设置：
   - 日期：每日
   - 时间：每5分钟
   
   任务设置：
   - 运行命令：
   ```
   
   监控脚本内容：
   ```bash
   #!/bin/bash
   if ! pgrep -f trade_alert_rust > /dev/null; then
       echo "$(date): 重启应用" >> /volume1/apps/trade-alert/logs/monitor.log
       cd /volume1/apps/trade-alert && ./start.sh &
   fi
   ```

## 🌐 第六部分：安全的外网访问（可选）

### 步骤10：配置反向代理

如果需要外网访问：

1. **配置反向代理**
   ```
   DSM → 控制面板 → 应用程序门户 → 反向代理 → 新增
   
   来源：
   - 协议：HTTPS
   - 主机名：*
   - 端口：443
   
   目标：
   - 协议：HTTP
   - 主机名：127.0.0.1
   - 端口：8000
   ```

2. **配置 SSL 证书**
   ```
   DSM → 控制面板 → 安全性 → 证书
   新增证书 → Let's Encrypt
   ```

## ✅ 第七部分：验证部署

### 步骤11：完整测试

1. **重启 NAS 测试自启**
   ```
   DSM → 控制面板 → 信息中心 → 重新启动
   ```

2. **验证服务启动**
   ```bash
   ssh tradealert@群晖IP
   ps aux | grep trade_alert_rust
   ```

3. **测试功能**
   - 访问 `http://群晖IP:8000`
   - 创建测试预警
   - 验证邮件发送

## 🔒 安全检查清单

- [ ] SSH 只允许内网IP访问
- [ ] 使用专用用户，非root运行
- [ ] 服务只绑定本地IP (127.0.0.1)
- [ ] 配置文件权限正确 (600)
- [ ] 应用目录权限隔离
- [ ] 防火墙规则配置
- [ ] 定期备份数据库
- [ ] 监控脚本正常运行

## 🆘 故障排除

### 常见问题

1. **编译失败**
   - 确认安装了 cross 工具
   - 检查网络连接（需要下载依赖）

2. **上传失败** 
   - 确认 SSH 权限
   - 检查目录权限

3. **启动失败**
   - 检查日志：`tail -f logs/app.log`
   - 验证邮件配置

4. **无法访问**
   - 确认端口8000未被占用
   - 检查防火墙设置

需要我详细指导任何特定步骤吗？ 