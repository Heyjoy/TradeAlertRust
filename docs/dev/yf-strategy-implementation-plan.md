# YF 扬帆顶底战法实施方案

> **基于现有项目技术栈的功能拆解和实施计划**  
> **分析时间**: 2025-07-12  
> **项目版本**: v2.3+ (多市场监控系统)

## 📊 现有项目技术能力分析

### ✅ 已有核心能力

#### 1. 数据基础设施
- **价格数据**: Yahoo Finance API (美股) + 腾讯 API (A股) + 加密货币
- **历史数据**: price_history 表支持 OHLCV 数据存储
- **实时监控**: 30秒间隔的价格获取机制
- **数据缓存**: 内存缓存 + 数据库持久化

#### 2. 技术架构
- **后端**: Rust + Axum + SQLx + SQLite
- **监控引擎**: 基于条件的实时价格监控
- **通知系统**: SMTP 邮件通知
- **多市场支持**: 美股、A股、加密货币

#### 3. 数据模型
```rust
// 现有核心数据结构
price_history: {
    symbol, date, open_price, high_price, low_price, 
    close_price, volume, daily_change_percent, volume_ratio
}

technical_signals: {
    symbol, indicator_type, signal_value, 
    signal_strength, description
}

market_anomalies: {
    symbol, anomaly_type, current_price, 
    change_percent, volume_ratio, severity
}
```

### ⚠️ 当前限制
- **无分钟级数据**: 只有日级别的历史数据
- **无基本面数据**: 缺少财务指标、行业数据
- **技术指标有限**: 只有简单的价格和成交量分析
- **无板块分类**: 缺少行业板块轮动分析

## 🔧 YF 战法功能拆解

### 核心战法细分

#### 1. 涨停回踩战法 (优先级: ⭐⭐⭐⭐⭐)
**功能点拆解**:
```rust
// 可立即实现 (基于现有数据)
✅ 涨停识别: daily_change_percent >= 9.8%
✅ 涨停时间统计: 基于价格变化历史
✅ 回踩检测: price < min(limit_up_low, prev_close)
✅ 缩量判断: current_volume < limit_up_volume * 0.5

// 需要增强 (简单扩展)
🔄 涨停质量评估: 开板次数、成交量分析
🔄 支撑位计算: 前期高点、均线支撑
```

**技术实现**:
```rust
pub struct LimitUpPullbackDetector {
    pub fn detect_limit_up(&self, prices: &[PriceData]) -> Option<LimitUpInfo> {
        // 检测涨停：涨幅 >= 9.8% (A股) 或自定义阈值
    }
    
    pub fn detect_pullback(&self, limit_up_info: &LimitUpInfo, current_price: f64) -> bool {
        // 检测回踩：当前价格低于涨停日最低价
    }
    
    pub fn check_volume_shrink(&self, current_vol: u64, limit_up_vol: u64) -> bool {
        // 检测缩量：当前量能 < 涨停日量能的50%
    }
}
```

#### 2. 底部突破战法 (优先级: ⭐⭐⭐⭐)
**功能点拆解**:
```rust
// 可立即实现
✅ 底部识别: price 接近 MA120/MA250
✅ 放量突破: volume > avg_volume * 2.0
✅ 价格突破: price > resistance_level
✅ 相对位置: (price - min) / (max - min) < 0.3

// 需要历史数据支持
🔄 箱体区间: 60日高低点统计
🔄 压力位识别: 历史成交密集区
```

**技术实现**:
```rust
pub struct BottomBreakoutDetector {
    pub fn is_bottom_stock(&self, prices: &[PriceData]) -> bool {
        let ma120 = self.calculate_ma(prices, 120);
        let ma250 = self.calculate_ma(prices, 250);
        let current = prices.last().unwrap().close_price;
        
        // 价格接近长期均线
        (current - ma120).abs() / ma120 < 0.15 &&
        (current - ma250).abs() / ma250 < 0.15
    }
    
    pub fn detect_breakout(&self, prices: &[PriceData]) -> Option<BreakoutSignal> {
        // 检测放量突破
    }
}
```

#### 3. 基础技术指标 (优先级: ⭐⭐⭐⭐⭐)
**功能点拆解**:
```rust
// 立即可实现的指标
✅ 移动平均: MA5, MA10, MA20, MA60, MA120, MA250
✅ 价格位置: 相对于均线的位置百分比
✅ 成交量比: 当前量/平均量
✅ 涨跌幅统计: 日、周、月涨跌幅
✅ 波动率: 价格波动程度计算

// 简化版高级指标
🔄 相对强弱: 与大盘对比强度
🔄 量价配合: 放量上涨/缩量下跌
🔄 趋势强度: 均线排列状态
```

**技术实现**:
```rust
pub struct TechnicalIndicators {
    pub fn calculate_moving_averages(&self, prices: &[f64]) -> MovingAverages {
        MovingAverages {
            ma5: self.sma(prices, 5),
            ma10: self.sma(prices, 10),
            ma20: self.sma(prices, 20),
            ma60: self.sma(prices, 60),
            ma120: self.sma(prices, 120),
            ma250: self.sma(prices, 250),
        }
    }
    
    pub fn calculate_price_position(&self, current: f64, mas: &MovingAverages) -> f64 {
        // 计算价格在均线系统中的相对位置
    }
    
    pub fn calculate_volume_ratio(&self, current_vol: u64, avg_vol: u64) -> f64 {
        current_vol as f64 / avg_vol as f64
    }
}
```

#### 4. 简化版庄影指标 (优先级: ⭐⭐⭐)
**功能点拆解**:
```rust
// 可以简化实现的部分
🔄 主力控盘度: EMA(EMA(close,13),13) 变化率
🔄 资金流向: 基于价格和成交量的估算
🔄 控盘状态: 有庄/无庄/出货状态判断

// 暂时无法实现 (需要逐笔数据)
❌ 精确筹码分析: WINNER函数
❌ 主动买卖分析: 需要逐笔成交数据
```

**技术实现**:
```rust
pub struct SimplifiedZhuangDetector {
    pub fn calculate_control_degree(&self, prices: &[f64]) -> f64 {
        let ema13 = self.ema(prices, 13);
        let ema13_ema13 = self.ema(&ema13, 13);
        
        // 计算控盘度变化率
        let current = ema13_ema13.last().unwrap();
        let prev = ema13_ema13[ema13_ema13.len()-2];
        
        (current - prev) / prev * 1000.0
    }
    
    pub fn detect_zhuang_status(&self, control_degree: f64) -> ZhuangStatus {
        match control_degree {
            x if x < -0.5 => ZhuangStatus::NoZhuang,
            x if x > 0.5 => ZhuangStatus::HasZhuang,
            _ => ZhuangStatus::Sideways,
        }
    }
}
```

## 🎯 最小可行版本 (MVP) 设计

### Phase 1: 基础版本 (1-2周实现)

#### 1.1 涨停回踩检测器
```rust
// 新增模块: src/services/strategy_analyzer.rs
pub struct StrategyAnalyzer {
    pub limit_up_pullback: LimitUpPullbackDetector,
    pub bottom_breakout: BottomBreakoutDetector,
    pub technical_indicators: TechnicalIndicators,
}

impl StrategyAnalyzer {
    pub async fn analyze_symbol(&self, symbol: &str) -> Result<StrategySignals> {
        // 获取历史数据
        let prices = self.get_price_history(symbol, 250).await?;
        
        // 执行各种策略分析
        let mut signals = StrategySignals::new();
        
        // 涨停回踩分析
        if let Some(pullback_signal) = self.limit_up_pullback.analyze(&prices) {
            signals.add_signal(Signal::LimitUpPullback(pullback_signal));
        }
        
        // 底部突破分析  
        if let Some(breakout_signal) = self.bottom_breakout.analyze(&prices) {
            signals.add_signal(Signal::BottomBreakout(breakout_signal));
        }
        
        Ok(signals)
    }
}
```

#### 1.2 策略信号存储
```sql
-- 新增策略信号表
CREATE TABLE strategy_signals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    strategy_type TEXT NOT NULL,        -- 'limit_up_pullback', 'bottom_breakout', etc.
    signal_strength INTEGER NOT NULL,   -- 1-5 信号强度
    trigger_price REAL NOT NULL,
    key_levels TEXT,                    -- JSON: 关键价位信息
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME                 -- 信号有效期
);

CREATE INDEX idx_strategy_signals_symbol_type ON strategy_signals(symbol, strategy_type);
CREATE INDEX idx_strategy_signals_created ON strategy_signals(created_at);
```

#### 1.3 前端界面集成
```rust
// 扩展现有模板: templates/strategy.html
#[derive(Template)]
#[template(path = "strategy.html")]
pub struct StrategyTemplate {
    pub market: Market,
    pub signals: Vec<StrategySignalView>,
    pub technical_analysis: TechnicalAnalysisView,
}

#[derive(Debug)]
pub struct StrategySignalView {
    pub symbol: String,
    pub name: String,
    pub strategy_type: String,
    pub signal_strength: u8,
    pub current_price: f64,
    pub key_levels: Vec<f64>,
    pub description: String,
    pub created_time: String,
}
```

#### 1.4 API 端点扩展
```rust
// 新增策略相关 API: src/handlers/strategy.rs
pub async fn get_strategy_signals(
    Path(market): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<StrategySignal>>, StatusCode> {
    // 返回指定市场的策略信号
}

pub async fn analyze_symbol_strategy(
    Path((market, symbol)): Path<(String, String)>,
    State(app_state): State<AppState>,
) -> Result<Json<StrategyAnalysisResult>, StatusCode> {
    // 对特定股票进行策略分析
}
```

### Phase 2: 增强版本 (2-3周实现)

#### 2.1 更多技术指标
```rust
pub struct AdvancedIndicators {
    pub fn calculate_rsi(&self, prices: &[f64], period: usize) -> f64 {
        // RSI 相对强弱指标
    }
    
    pub fn detect_golden_cross(&self, short_ma: &[f64], long_ma: &[f64]) -> bool {
        // 金叉检测
    }
    
    pub fn calculate_bollinger_bands(&self, prices: &[f64], period: usize) -> BollingerBands {
        // 布林带指标
    }
    
    pub fn detect_volume_surge(&self, volumes: &[u64], period: usize) -> bool {
        // 放量检测
    }
}
```

#### 2.2 策略组合评分
```rust
pub struct StrategyScorer {
    pub fn calculate_composite_score(&self, signals: &[Signal]) -> CompositeScore {
        let mut score = 0.0;
        let mut weight_sum = 0.0;
        
        for signal in signals {
            let (signal_score, weight) = match signal {
                Signal::LimitUpPullback(s) => (s.strength as f64 * 0.8, 1.0),
                Signal::BottomBreakout(s) => (s.strength as f64 * 0.6, 0.8),
                Signal::TechnicalIndicator(s) => (s.value, 0.4),
                // ... 其他信号
            };
            
            score += signal_score * weight;
            weight_sum += weight;
        }
        
        CompositeScore {
            total_score: score / weight_sum,
            signal_count: signals.len(),
            confidence: self.calculate_confidence(signals),
        }
    }
}
```

## 🚀 立即可实现的功能清单

### 优先级 1: 核心战法 (本周完成)
1. ✅ **涨停检测**: 基于 daily_change_percent 字段
2. ✅ **回踩判断**: 价格相对涨停日低点的位置
3. ✅ **缩量确认**: 当前成交量与历史成交量对比
4. ✅ **移动平均计算**: MA5/10/20/60/120/250
5. ✅ **底部识别**: 价格接近长期均线程度

### 优先级 2: 用户界面 (下周完成)
1. ✅ **策略页面**: 新增 /strategy/{market} 路由
2. ✅ **信号展示**: 策略信号列表和详情
3. ✅ **技术分析图表**: 简单的价格和指标展示
4. ✅ **邮件通知集成**: 策略信号触发邮件

### 优先级 3: 数据增强 (第3周)
1. 🔄 **历史数据补全**: 确保有足够的价格历史
2. 🔄 **成交量分析**: 量比、换手率等指标
3. 🔄 **相对强度**: 个股与大盘的对比强度

## 💻 具体实现步骤

### Step 1: 创建策略分析模块
```bash
# 1. 创建新的策略分析服务
touch src/services/strategy_analyzer.rs

# 2. 更新模块导出
# 在 src/lib.rs 中添加: pub mod strategy_analyzer;

# 3. 创建数据库迁移
# migrations/20250712000002_add_strategy_signals.sql
```

### Step 2: 实现基础指标计算
```rust
// src/services/strategy_analyzer.rs
pub struct TechnicalCalculator;

impl TechnicalCalculator {
    pub fn simple_moving_average(prices: &[f64], period: usize) -> Option<f64> {
        if prices.len() < period {
            return None;
        }
        
        let sum: f64 = prices.iter().rev().take(period).sum();
        Some(sum / period as f64)
    }
    
    pub fn detect_limit_up(prev_close: f64, current_high: f64, threshold: f64) -> bool {
        let change_percent = (current_high - prev_close) / prev_close * 100.0;
        change_percent >= threshold
    }
    
    pub fn calculate_volume_ratio(current_vol: u64, avg_vol: u64) -> f64 {
        if avg_vol == 0 { return 0.0; }
        current_vol as f64 / avg_vol as f64
    }
}
```

### Step 3: 集成到现有监控系统
```rust
// src/services/fetcher.rs 中添加策略分析
impl PriceFetcher {
    async fn check_strategy_signals(&self, symbol: &str, price_data: &StockPrice) -> Result<()> {
        let analyzer = StrategyAnalyzer::new(&self.db);
        
        if let Ok(signals) = analyzer.analyze_symbol(symbol).await {
            for signal in signals.iter() {
                if signal.strength >= 4 {  // 高强度信号
                    self.send_strategy_notification(symbol, signal).await?;
                }
            }
        }
        
        Ok(())
    }
}
```

### Step 4: 创建前端展示界面
```html
<!-- templates/strategy.html -->
<div class="strategy-signals">
    {% for signal in signals %}
    <div class="signal-card signal-{{signal.strategy_type}}">
        <h3>{{signal.symbol}} - {{signal.name}}</h3>
        <div class="signal-strength">
            强度: {% for i in range(end=signal.signal_strength) %}⭐{% endfor %}
        </div>
        <div class="signal-description">{{signal.description}}</div>
        <div class="key-levels">
            关键位: {% for level in signal.key_levels %}{{level}}{% endfor %}
        </div>
    </div>
    {% endfor %}
</div>
```

## 📊 预期效果

### 短期目标 (2周内)
- ✅ 实现涨停回踩战法自动检测
- ✅ 提供基础的技术指标分析
- ✅ 集成到现有的监控和通知系统
- ✅ 用户可通过 Web 界面查看策略信号

### 中期目标 (1个月内)  
- 🔄 覆盖4种核心战法的基础版本
- 🔄 提供策略信号的历史回测功能
- 🔄 优化信号准确率和及时性
- 🔄 增加更多技术指标支持

### 长期目标 (3个月内)
- 📅 接入分钟级数据，提升信号精度
- 📅 增加基本面数据，完善选股逻辑
- 📅 开发策略回测和优化系统
- 📅 支持用户自定义策略参数

## 💡 创新优化建议

### 1. 利用现有优势
- **多市场覆盖**: 将战法应用到美股和加密货币市场
- **实时监控**: 结合现有的 30 秒监控机制
- **通知集成**: 策略信号自动邮件推送

### 2. 技术创新
- **机器学习**: 使用历史数据优化策略参数
- **风险控制**: 增加止损和仓位管理建议
- **用户反馈**: 收集策略效果反馈，持续优化

### 3. 商业价值
- **差异化竞争**: 独特的技术指标组合
- **用户粘性**: 专业的策略分析能力
- **商业化潜力**: 可作为付费服务功能

---

**实施建议**: 优先实现涨停回踩战法，这个战法逻辑清晰、数据需求简单，可以快速验证技术路线的可行性，为后续扩展打下基础。