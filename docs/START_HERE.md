# 🚀 AI助手启动指南 (每次对话必读)

> 📋 **重要**: 这是每次对话的第一个文件，请按顺序执行以下步骤

## 🎯 **快速启动流程**

### Step 1: 读取核心状态 (必读)
请按顺序读取以下文件建立基础上下文：

1. **[当前状态](docs/Requirement/CURRENT_STATUS.md)** - 了解当前讨论重点和待决策问题
2. **[项目导航](docs/Requirement/README.md)** - 掌握项目概况和文档结构
3. **[工作模式](docs/Requirement/0.0-REQUIREMENT_MANAGEMENT_STRATEGY.md)** - 理解ss协作方式
4. **[配置管理规则](docs/technical/CONFIGURATION_MANAGEMENT.md)** - 掌握配置命名规则和文件位置
5. **[TODO看板分工规则](../TODO看板分工规则标准.md)** - 理解GTD看板协作模式

### Step 2: 检查TODO看板状态 (必读)
读取并理解当前任务状态：

6. **[TODO看板](../TODO.md)** - 查看当前任务分布和优先级
   - **INBOX Processing**: 待分类的新任务
   - **Todo**: 用户待处理的任务
   - **ME In Progress**: 用户正在执行的任务
   - **AI In Discussion**: 需要AI和用户讨论的任务
   - **AI In Progress**: AI可直接执行的任务
   - **SomedayMaybe**: 未来可能的功能
   - **Done**: 已完成的任务

### Step 3: 根据话题读取专项文档 (按需)
根据用户提到的话题，选择性读取：

#### 如果讨论A股策略开发
- **[策略引擎需求](docs/Requirement/2.3-STRATEGY_ENGINE.md)**
- **[涨停回踩规格](docs/Requirement/3.1-LIMIT_UP_PULLBACK.md)**

#### 如果讨论多市场支持
- **[多市场需求](docs/Requirement/2.1-MULTI_MARKET.md)**

#### 如果讨论移动端优化
- **[移动端需求](docs/Requirement/2.2-MOBILE_DESIGN.md)**

#### 如果讨论技术实现
- **[开发计划](docs/development/DEVELOPMENT_PLAN.md)**
- **[配置管理规则](docs/technical/CONFIGURATION_MANAGEMENT.md)** - 环境变量命名和配置文件位置

#### 如果讨论产品规划
- **[PRD主文档](docs/Requirement/1.1-PRD_MASTER.md)**

### Step 4: 确认理解并开始对话
读取完成后，简要总结：
- 当前项目状态
- TODO看板中的关键任务
- 主要讨论话题
- 待解决的关键问题
- 准备好协助的具体事项

## 📊 **项目基本信息** (快速参考)

### 项目概况
- **名称**: TradeAlert - 智能交易预警系统
- **当前版本**: v1.0 (美股预警) 已完成，v2.1 (A股策略) 开发中
- **技术栈**: Rust + Axum + SQLite + Yahoo Finance API
- **部署**: Railway云平台

### 团队协作
- **用户角色**: 产品决策 + 需求澄清 + 功能验收 + 任务优先级管理
- **AI角色**: 文档编写 + 代码实现 + 技术方案 + 任务执行
- **协作模式**: 用户"口花花"描述，AI快速响应和实现
- **任务管理**: GTD看板系统，明确分工边界

### 当前重点 (2025-07-12)
- **主要任务**: 用户体验优化和功能完善
- **核心功能**: 多市场监控、预警系统、邮件通知
- **关键决策**: 技术债务处理、性能优化、新功能开发

## 🎭 **常见对话场景**

### 场景1: 继续上次讨论
```
用户: "我们继续A股策略的讨论"
AI: [读取CURRENT_STATUS + TODO看板 + 2.3-STRATEGY_ENGINE] 
    "了解了，当前TODO看板中有市场异动监控系统架构设计任务，需要决策A股数据源选择和策略参数..."
```

### 场景2: 新的需求讨论
```
用户: "我想讨论移动端优化的需求"
AI: [读取CURRENT_STATUS + TODO看板 + 2.2-MOBILE_DESIGN]
    "好的，TODO看板中有设计移动端用户体验改进方案任务，当前状态是..."
```

### 场景3: 技术实现问题
```
用户: "看看开发计划，我们实现一下A股数据集成"
AI: [读取DEVELOPMENT_PLAN + TODO看板 + 相关技术文档]
    "明白了，根据开发计划和TODO看板，A股数据集成需要..."
```

### 场景4: 任务管理讨论
```
用户: "更新一下TODO看板"
AI: [读取TODO看板 + 分工规则]
    "好的，我来检查和更新TODO看板，根据分工规则..."
```

### 场景5: AI任务执行
```
用户: "执行AI In Progress中的任务"
AI: [读取TODO看板 + 相关技术文档]
    "好的，我将按顺序执行AI In Progress中的任务：清理编译警告、完善错误处理、更新项目文档..."
```

## ⚡ **高效对话技巧**

### 用户提示词建议
- **明确话题**: "我们讨论A股策略" vs "随便聊聊"
- **指定文档**: "看看2.3文档" vs "看看策略相关的"
- **明确目标**: "我要做决策" vs "我要写代码"
- **任务管理**: "更新TODO看板" vs "看看有什么任务"

### AI响应模式
- **快速确认**: 总结理解的重点和TODO看板状态
- **主动建议**: 基于文档和任务状态提出下一步行动
- **并行处理**: 同时处理多个相关任务
- **任务跟踪**: 及时更新TODO看板状态

## 🔧 **文档维护提醒**

### 每次对话后更新
- **CURRENT_STATUS.md** - 更新讨论重点和决策状态
- **TODO.md** - 更新任务状态和进度
- **相关需求文档** - 根据讨论结果更新具体需求

### 定期维护
- **DEVELOPMENT_PLAN.md** - 技术实现进展
- **README.md** - 项目状态和导航信息
- **TODO看板分工规则标准.md** - 协作规则优化

### 重大变更时更新
- **PRD_MASTER.md** - 产品方向调整
- **0.0-REQUIREMENT_MANAGEMENT_STRATEGY.md** - 工作模式优化

## 📋 **GTD看板工作流程**

### TODO看板使用指南
📋 **详细规则**: 参见 `../TODO看板分工规则标准.md`

#### 🎯 任务分工原则
- **AI In Discussion**: 需要AI启动讨论的任务，用户确认后移入Progress
- **AI In Progress**: 用户已确认，AI可直接执行的任务，按排列顺序执行
- **其他列**: 用户负责做决定的任务，AI可根据指令和现状帮忙撰写

#### 🔄 任务流转规则
```
INBOX Processing → Todo → ME In Progress → Done
     ↓
AI In Discussion → AI In Progress → Done
     ↓
Todo → SomedayMaybe (暂不重要)
```

#### 📝 格式规范
- **每个任务必须是一句话**: 避免使用分割符、换行或复杂格式
- **简洁明了**: 任务描述要清晰，避免冗长的说明
- **统一风格**: 所有列使用相同的格式标准
- **信息完整**: 在简洁的前提下包含必要信息

#### 🎭 AI协作行为规范
1. **任务建议时**: 明确说明建议理由，提供具体执行方案，预估执行时间
2. **任务执行时**: 严格按照用户确认内容执行，保持执行过程透明
3. **任务完成时**: 记录完整执行结果，总结成果和价值，提供后续建议

## 🤖 **AI协作工作流程**

### 完整协作流程指南
📋 **详细说明**: 参见 `docs/ai-collaboration-workflow.md`

#### 🚀 工作会话启动
```bash
# 运行启动脚本进行环境检查和上下文加载
scripts/development/start_work_session.bat
```

#### 🎭 AI专家模式选择
根据任务类型选择合适的AI专家模式：
- `re` - Rust Expert (核心功能开发)
- `ta` - Trading Analyst (交易逻辑实现)
- `sa` - Security Auditor (安全审查)
- `do` - DevOps Engineer (部署运维)
- `ar` - System Architect (架构设计)
- `dw` - Documentation Writer (文档编写)
- `qa` - QA Engineer (测试质量保证)

#### 🔄 标准开发循环
1. **任务分析** - 需求澄清、方案设计、风险识别
2. **代码实现** - 遵循规范、实时审查、文档同步
3. **验证确认** - 质量检查、功能验证、文档验证
4. **任务更新** - 更新TODO看板状态，记录完成情况

#### 📊 工作会话收尾
```bash
# 运行收尾脚本进行质量检查和状态总结
scripts/development/end_work_session.bat
```

---

## 📝 **使用说明**

### 对用户
每次对话开始时，可以选择：
- **需求讨论**: "读一下START_HERE，我们开始工作"
- **开发会话**: "运行start_work_session.bat，开始编码"
- **专项任务**: "按启动流程，我们继续A股策略讨论"
- **任务管理**: "更新TODO看板，看看当前任务状态"

### 对AI
收到指令后：
1. 读取START_HERE.md (本文档)
2. 按流程读取核心文档
3. 检查TODO看板状态
4. 根据话题读取专项文档
5. 确认理解并开始协作

## 📋 **重要参考文档**

### **🏗️ 架构和设计**
- **[项目架构文档](PROJECT_ARCHITECTURE.md)** - 完整系统架构和技术栈
- **[AI协作工作流程](ai-collaboration-workflow.md)** - 详细协作流程指南
- **[AI协作经验沉淀](dev/ai-collaboration-insights.md)** - 实战协作心得和最佳实践
- **[配置管理规则](technical/CONFIGURATION_MANAGEMENT.md)** - 配置规范

### **📊 项目管理**
- **[开发状态跟踪](development-status.md)** - 项目整体进展
- **[当前任务管理](../tasks/current-tasks.md)** - 任务跟踪系统
- **[Phase1完成总结](phase1-completion-summary.md)** - 里程碑记录
- **[TODO看板](../TODO.md)** - GTD任务管理系统
- **[TODO看板分工规则](../TODO看板分工规则标准.md)** - 协作规则标准

---

**文档版本**: v1.2  
**创建时间**: 2025-06-14  
**最后更新**: 2025-07-12  
**维护人**: AI助手  
**更新频率**: 根据工作模式变化调整 