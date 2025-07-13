# TradeAlert 演示模式使用指南

> 🔬 **演示环境** - 安全的朋友测试和功能演示环境

## 🎯 **演示模式概述**

TradeAlert演示模式是一个安全的测试环境，专为朋友测试和功能演示设计。在演示模式下，每个用户的数据完全隔离，不会影响生产数据。

### **演示模式特性**
- ✅ **用户数据隔离** - 每个访问者看到独立的预警数据
- ✅ **数量限制保护** - 每用户最多5个预警，防止滥用
- ✅ **数据自动清理** - 24小时后自动删除测试数据
- ✅ **邮件通知禁用** - 不会发送真实邮件通知
- ✅ **功能完整体验** - 除邮件外所有功能正常工作
- ✅ **独立数据库** - 使用专门的演示数据库

## 🚀 **如何启动演示模式**

### **方法一：使用启动脚本（推荐）**

**Windows用户：**
```powershell
# 在项目根目录执行
.\scripts\start_demo.ps1
```

**Linux/macOS用户：**
```bash
# 在项目根目录执行
./scripts/start_demo.sh
```

### **方法二：手动配置环境变量**

```bash
# 设置环境变量
export TRADE_ALERT__DEMO__ENABLED=true
export TRADE_ALERT__DEMO__MAX_ALERTS_PER_USER=5
export TRADE_ALERT__EMAIL__ENABLED=false
export DATABASE_URL=sqlite:data/demo_trade_alert.db

# 启动应用
cargo run --bin trade_alert_rust
```

### **方法三：使用配置文件**
```bash
# 复制演示配置
cp config/.env.demo .env

# 启动应用
cargo run --bin trade_alert_rust
```

## 🌐 **分享给朋友测试**

### **本地网络分享**
```bash
# 启动后在浏览器访问
http://localhost:3000?demo=true

# 如果需要局域网访问，修改配置：
export TRADE_ALERT__SERVER__HOST=0.0.0.0
# 然后朋友可以通过您的IP访问：
http://YOUR_IP:3000?demo=true
```

### **使用ngrok公网分享**
```bash
# 安装ngrok后执行
ngrok http 3000

# 将生成的公网URL分享给朋友
https://abc123.ngrok.io?demo=true
```

### **Railway临时部署**
```bash
# 使用演示配置部署到Railway
railway up --environment demo
```

## 👥 **用户使用指南**

### **首次访问**
1. 打开演示链接 `http://localhost:3000?demo=true`
2. 系统自动生成唯一用户ID
3. 看到演示模式横幅提示
4. 开始创建和测试预警

### **功能体验**
- ✅ **创建预警** - 可以正常创建各类预警
- ✅ **价格监控** - 实时价格更新正常工作
- ✅ **市场切换** - 美股、A股、加密货币切换
- ✅ **预警管理** - 查看、编辑、删除预警
- ⚠️ **邮件通知** - 显示"发送成功"但不会真实发送

### **演示限制**
- 每个用户最多5个预警
- 数据24小时后自动清理
- 邮件通知仅模拟，不实际发送
- API请求频率限制（20次/分钟）

## 🔧 **技术实现原理**

### **用户隔离机制**
```javascript
// 前端自动生成用户ID
const userId = localStorage.getItem('trade_alert_user_id') || generateUserId();

// 所有API请求自动添加用户标识头
headers['X-User-Id'] = userId;
```

### **后端数据隔离**
```rust
// 数据库查询自动过滤用户数据
SELECT * FROM alerts WHERE user_id = ? AND status = 'active'

// 创建预警时自动分配用户ID
INSERT INTO alerts (symbol, condition, price, user_id) VALUES (?, ?, ?, ?)
```

### **演示模式配置**
```rust
pub struct DemoConfig {
    pub enabled: bool,                    // 是否启用演示模式
    pub max_alerts_per_user: u32,         // 每用户最大预警数
    pub data_retention_hours: u64,        // 数据保留时间
    pub disable_email: bool,              // 禁用邮件发送
    pub show_demo_banner: bool,           // 显示演示横幅
    pub rate_limit_per_minute: u32,       // 频率限制
}
```

## 🛡️ **安全和隐私保护**

### **数据安全**
- ✅ **完全隔离** - 演示数据与生产数据完全分离
- ✅ **自动清理** - 定期清理过期演示数据
- ✅ **无敏感信息** - 不收集真实个人信息
- ✅ **本地存储** - 用户ID仅存储在浏览器本地

### **隐私保护**
- ✅ **匿名访问** - 无需注册或登录
- ✅ **临时标识** - 用户ID为临时生成的随机字符串
- ✅ **数据不持久** - 演示数据定期自动删除
- ✅ **无跟踪** - 不进行用户行为跟踪

## 📱 **移动端使用**

演示模式完全支持移动端访问：
- 📱 **响应式设计** - 自适应手机和平板屏幕
- 👆 **触控优化** - 针对触控操作优化界面
- 🔄 **同步状态** - 多设备间数据隔离但体验一致

## 🆘 **常见问题**

### **Q: 演示模式安全吗？**
A: 非常安全。演示模式使用独立数据库，数据完全隔离，且会定期自动清理。

### **Q: 可以同时多人测试吗？**
A: 可以。每个访问者都有独立的用户空间，互不干扰。

### **Q: 为什么收不到邮件？**
A: 演示模式禁用了真实邮件发送，避免发送垃圾邮件。界面会显示"发送成功"但实际不发送。

### **Q: 数据会被保存吗？**
A: 演示数据会在24小时后自动删除，不会长期保存。

### **Q: 可以导入真实数据测试吗？**
A: 不建议。演示模式仅用于功能体验，不应包含真实的投资数据。

## 🔄 **从演示模式切换到生产模式**

如果朋友测试满意，想要正式使用：

1. **停止演示模式**
   ```bash
   # 按 Ctrl+C 停止演示模式
   ```

2. **启动生产模式**
   ```bash
   # 使用正常启动脚本
   .\scripts\dev_start.ps1
   # 或
   cargo run --bin trade_alert_rust
   ```

3. **配置邮件通知**
   ```bash
   # 复制环境变量模板
   cp config/_env.example .env
   # 编辑 .env 文件，配置真实的邮件设置
   ```

## 📋 **演示清单**

### **向朋友演示时的建议流程：**

1. ✅ **启动演示环境** - 运行 `start_demo.ps1`
2. ✅ **打开演示链接** - `http://localhost:3000?demo=true`
3. ✅ **展示多市场支持** - 切换美股/A股/加密货币
4. ✅ **创建测试预警** - 演示预警创建流程
5. ✅ **实时价格更新** - 展示价格监控功能
6. ✅ **预警管理** - 展示编辑、删除功能
7. ✅ **移动端体验** - 在手机上打开测试
8. ✅ **解释隐私保护** - 说明数据隔离和自动清理

---

**📝 文档维护**: 开发团队  
**🔄 最后更新**: 2025-07-12  
**📅 适用版本**: v2.3+ (演示模式支持)