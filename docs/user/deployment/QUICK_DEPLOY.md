# 🚀 5分钟快速部署指南

## 当前状态
✅ 系统已启动：http://localhost:3000
✅ 邮件功能正常
✅ 价格监控运行中

## 立即创建公网访问

### 🔐 重要提示：ngrok 需要免费注册
ngrok 现在需要免费账户才能使用。请选择以下方案之一：

### 方案A：注册 ngrok（推荐，稳定）

#### 1. 安装 ngrok
```bash
winget install ngrok
```

#### 2. 注册并设置
1. **注册账户**：https://dashboard.ngrok.com/signup
2. **获取 authtoken**：https://dashboard.ngrok.com/get-started/your-authtoken
3. **设置 authtoken**：
   ```bash
   ngrok config add-authtoken YOUR_AUTHTOKEN_HERE
   ```
4. **创建隧道**：
   ```bash
   ngrok http 3000
   ```

### 方案B：使用免费替代工具

#### LocalTunnel（无需注册）
```bash
# 安装
npm install -g localtunnel

# 使用
lt --port 3000
```

#### Serveo（无需安装）
```bash
ssh -R 80:localhost:3000 serveo.net
```

#### Telebit（稳定性好）
```bash
# 安装
npm install -g @telebit/cli

# 使用
telebit http 3000
```

## 🚀 推荐流程

### 快速测试（选择 LocalTunnel）
```bash
npm install -g localtunnel
lt --port 3000
```

### 长期使用（选择 ngrok）
```bash
# 1. 注册 ngrok 账户
# 2. 设置 authtoken
ngrok config add-authtoken YOUR_TOKEN
# 3. 使用
ngrok http 3000
```

## 创建公网隧道

### 基础版本（免费）
```bash
ngrok http 3000
```

### 自定义域名版本（需要账户）
```bash
ngrok http 3000 --domain=your-custom-name.ngrok.app
```

## 获取公网URL

运行ngrok后，您会看到：
```
ngrok                                                       (Ctrl+C to quit)

Session Status                online
Account                       Free (Limit: 1 tunnel)
Version                       3.0.0
Region                        Asia Pacific (ap)
Latency                       -
Web Interface                 http://127.0.0.1:4040
Forwarding                    https://abc123.ngrok.app -> http://localhost:3000

Connections                   ttl     opn     rt1     rt5     p50     p90
                              0       0       0.00    0.00    0.00    0.00
```

**您的公网地址就是：https://abc123.ngrok.app**

## 📧 发给朋友的测试邀请

复制以下内容发给朋友：

---

**嗨！帮我测试一个股票预警系统，2分钟搞定！🚀**

**测试地址**：https://your-ngrok-url.ngrok.app

**测试步骤**：
1. 打开网址 
2. 点击「创建新预警」
3. 填写信息：
   - 股票代码：`AAPL`
   - 选择：`低于` 当前价格
   - 目标价格：比当前价低$1（容易触发）
   - 邮箱：你的邮箱地址
4. 点击「创建预警」
5. 等待1-5分钟检查邮箱

**预期结果**：
- ✅ 你会收到HTML格式的股票预警邮件
- ✅ 邮件包含：AAPL价格信息、触发时间等

**请反馈**：
1. 网站是否正常打开？
2. 表单提交是否成功？  
3. 邮件是否收到？
4. 邮件内容是否清晰？
5. 有什么改进建议？

感谢帮忙测试！🙏

---

## 🎯 测试监控

在ngrok运行期间，您可以：

1. **查看访问日志**：http://127.0.0.1:4040
2. **监控系统日志**：查看cargo运行窗口
3. **测试邮件功能**：`cargo run --bin test_email`

## 📊 成功指标

- [ ] 朋友能正常打开网站
- [ ] 表单提交成功
- [ ] 预警创建成功  
- [ ] 邮件发送成功
- [ ] 朋友收到邮件
- [ ] 反馈积极

## 🛠️ 如果遇到问题

### ngrok连接问题
```bash
# 重启ngrok
Ctrl+C (停止ngrok)
ngrok http 3000 (重新启动)
```

### 服务器问题
```bash
# 重启服务器
Ctrl+C (停止cargo)
cargo run --bin trade_alert_rust (重新启动)
```

### 邮件问题
```bash
# 测试邮件配置
cargo run --bin test_email
```

---

**现在就开始吧！运行 `ngrok http 3000` 🚀** 