use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{info, warn};

/// 策略分析器 - 实现YF扬帆顶底战法
pub struct StrategyAnalyzer {
    db: SqlitePool,
}

/// 价格数据结构
#[derive(Debug, Clone)]
pub struct PriceData {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub change_percent: Option<f64>,
}

/// 策略信号类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StrategySignal {
    LimitUpPullback(LimitUpPullbackSignal),
    BottomBreakout(BottomBreakoutSignal),
    TechnicalIndicator(TechnicalSignal),
}

/// 涨停回踩信号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitUpPullbackSignal {
    pub symbol: String,
    pub signal_strength: u8,          // 1-5 信号强度
    pub limit_up_date: String,        // 涨停日期
    pub limit_up_price: f64,          // 涨停价格
    pub pullback_price: f64,          // 当前回踩价格
    pub volume_shrink_ratio: f64,     // 缩量比例
    pub support_level: f64,           // 支撑位
    pub description: String,
    pub key_levels: Vec<f64>,         // 关键价位
}

/// 底部突破信号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottomBreakoutSignal {
    pub symbol: String,
    pub signal_strength: u8,
    pub breakout_price: f64,
    pub volume_ratio: f64,
    pub ma_position: f64,             // 相对均线位置
    pub resistance_level: f64,        // 突破的阻力位
    pub description: String,
    pub key_levels: Vec<f64>,
}

/// 技术指标信号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalSignal {
    pub symbol: String,
    pub indicator_name: String,
    pub value: f64,
    pub signal_strength: u8,
    pub description: String,
}

/// 移动平均数据
#[derive(Debug, Clone)]
pub struct MovingAverages {
    pub ma5: Option<f64>,
    pub ma10: Option<f64>,
    pub ma20: Option<f64>,
    pub ma60: Option<f64>,
    pub ma120: Option<f64>,
    pub ma250: Option<f64>,
}

/// 涨停信息
#[derive(Debug, Clone)]
pub struct LimitUpInfo {
    pub date: String,
    pub price: f64,
    pub low_price: f64,
    pub volume: u64,
    pub change_percent: f64,
}

impl StrategyAnalyzer {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    /// 分析指定股票的策略信号
    pub async fn analyze_symbol(&self, symbol: &str) -> Result<Vec<StrategySignal>> {
        info!("开始分析股票策略信号: {}", symbol);
        
        // 获取历史价格数据
        let prices = self.get_price_history(symbol, 250).await?;
        if prices.len() < 60 {
            warn!("股票 {} 历史数据不足，无法进行策略分析", symbol);
            return Ok(vec![]);
        }

        let mut signals = Vec::new();

        // 1. 涨停回踩分析
        if let Some(signal) = self.analyze_limit_up_pullback(&prices).await? {
            signals.push(StrategySignal::LimitUpPullback(signal));
        }

        // 2. 底部突破分析
        if let Some(signal) = self.analyze_bottom_breakout(&prices).await? {
            signals.push(StrategySignal::BottomBreakout(signal));
        }

        // 3. 技术指标分析
        let tech_signals = self.analyze_technical_indicators(&prices).await?;
        for signal in tech_signals {
            signals.push(StrategySignal::TechnicalIndicator(signal));
        }

        info!("股票 {} 分析完成，生成 {} 个信号", symbol, signals.len());
        Ok(signals)
    }

    /// 获取历史价格数据
    async fn get_price_history(&self, symbol: &str, days: i32) -> Result<Vec<PriceData>> {
        let rows = sqlx::query!(
            r#"
            SELECT date, open_price, high_price, low_price, close_price, 
                   volume, daily_change_percent
            FROM price_history 
            WHERE symbol = ? 
            ORDER BY date DESC 
            LIMIT ?
            "#,
            symbol,
            days
        )
        .fetch_all(&self.db)
        .await?;

        let mut prices = Vec::new();
        for row in rows {
            prices.push(PriceData {
                date: row.date.to_string(),
                open: row.open_price,
                high: row.high_price,
                low: row.low_price,
                close: row.close_price,
                volume: row.volume as u64,
                change_percent: row.daily_change_percent,
            });
        }

        // 反转数组，让最新数据在最后
        prices.reverse();
        Ok(prices)
    }

    /// 涨停回踩分析
    async fn analyze_limit_up_pullback(&self, prices: &[PriceData]) -> Result<Option<LimitUpPullbackSignal>> {
        if prices.len() < 10 {
            return Ok(None);
        }

        // 1. 寻找近10天内的涨停
        let mut limit_up_info = None;
        for (_i, price) in prices.iter().enumerate().rev().take(10) {
            if let Some(change_percent) = price.change_percent {
                // A股涨停阈值 9.8%，美股可以设置为自定义阈值
                if change_percent >= 9.8 {
                    limit_up_info = Some(LimitUpInfo {
                        date: price.date.clone(),
                        price: price.high,
                        low_price: price.low,
                        volume: price.volume,
                        change_percent,
                    });
                    break;
                }
            }
        }

        let limit_up = match limit_up_info {
            Some(info) => info,
            None => return Ok(None),
        };

        // 2. 检查当前是否回踩
        let current_price = prices.last().unwrap();
        let is_pullback = current_price.close < limit_up.low_price.min(
            prices[prices.len().saturating_sub(2)].close
        );

        if !is_pullback {
            return Ok(None);
        }

        // 3. 检查缩量
        let volume_shrink_ratio = current_price.volume as f64 / limit_up.volume as f64;
        if volume_shrink_ratio > 0.5 {
            return Ok(None); // 没有明显缩量
        }

        // 4. 计算支撑位
        let mas = self.calculate_moving_averages(prices);
        let support_level = [mas.ma5, mas.ma10, mas.ma20]
            .iter()
            .filter_map(|&ma| ma)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(current_price.close * 0.95);

        // 5. 计算信号强度
        let mut strength = 3u8; // 基础强度
        if volume_shrink_ratio < 0.3 { strength += 1; } // 深度缩量加分
        if current_price.close > support_level { strength += 1; } // 仍在支撑位上方加分

        let signal = LimitUpPullbackSignal {
            symbol: prices[0].date.clone(), // 这里应该传入symbol，临时用date代替
            signal_strength: strength.min(5),
            limit_up_date: limit_up.date.clone(),
            limit_up_price: limit_up.price,
            pullback_price: current_price.close,
            volume_shrink_ratio,
            support_level,
            description: format!(
                "涨停回踩：{}涨停后回踩，缩量{:.1}%，当前价{:.2}，支撑位{:.2}",
                limit_up.date,
                (1.0 - volume_shrink_ratio) * 100.0,
                current_price.close,
                support_level
            ),
            key_levels: vec![support_level, limit_up.low_price, limit_up.price],
        };

        Ok(Some(signal))
    }

    /// 底部突破分析
    async fn analyze_bottom_breakout(&self, prices: &[PriceData]) -> Result<Option<BottomBreakoutSignal>> {
        if prices.len() < 120 {
            return Ok(None);
        }

        let current = prices.last().unwrap();
        let mas = self.calculate_moving_averages(prices);

        // 1. 检查是否为底部股票（接近长期均线）
        let ma120 = mas.ma120.unwrap_or(0.0);
        let ma250 = mas.ma250.unwrap_or(0.0);
        
        if ma120 == 0.0 || ma250 == 0.0 {
            return Ok(None);
        }

        let close_to_ma120 = (current.close - ma120).abs() / ma120 < 0.15;
        let close_to_ma250 = (current.close - ma250).abs() / ma250 < 0.15;

        if !close_to_ma120 || !close_to_ma250 {
            return Ok(None);
        }

        // 2. 检查放量突破
        let avg_volume = self.calculate_average_volume(prices, 30);
        let volume_ratio = current.volume as f64 / avg_volume;
        
        if volume_ratio < 1.5 {
            return Ok(None); // 没有明显放量
        }

        // 3. 计算阻力位（60日高点）
        let resistance_level = prices.iter()
            .rev()
            .take(60)
            .map(|p| p.high)
            .fold(0.0, f64::max);

        // 4. 检查是否突破阻力位
        if current.high <= resistance_level * 1.02 { // 需要突破2%以上
            return Ok(None);
        }

        // 5. 计算信号强度
        let mut strength = 3u8;
        if volume_ratio > 2.0 { strength += 1; }
        if current.close > resistance_level { strength += 1; }

        let signal = BottomBreakoutSignal {
            symbol: current.date.clone(), // 临时用date代替symbol
            signal_strength: strength.min(5),
            breakout_price: current.close,
            volume_ratio,
            ma_position: (current.close - ma250) / ma250 * 100.0,
            resistance_level,
            description: format!(
                "底部突破：放量{:.1}倍突破{:.2}阻力位，当前价{:.2}",
                volume_ratio,
                resistance_level,
                current.close
            ),
            key_levels: vec![ma120, ma250, resistance_level],
        };

        Ok(Some(signal))
    }

    /// 技术指标分析
    async fn analyze_technical_indicators(&self, prices: &[PriceData]) -> Result<Vec<TechnicalSignal>> {
        let mut signals = Vec::new();
        let mas = self.calculate_moving_averages(prices);
        let current = prices.last().unwrap();

        // 1. 均线多头排列检测
        if let (Some(ma5), Some(ma10), Some(ma20)) = (mas.ma5, mas.ma10, mas.ma20) {
            if ma5 > ma10 && ma10 > ma20 && current.close > ma5 {
                signals.push(TechnicalSignal {
                    symbol: current.date.clone(),
                    indicator_name: "均线多头".to_string(),
                    value: (current.close - ma20) / ma20 * 100.0,
                    signal_strength: 4,
                    description: "短期均线呈多头排列，价格位于均线上方".to_string(),
                });
            }
        }

        // 2. 量价配合分析
        let volume_ma = self.calculate_average_volume(prices, 5);
        let volume_ratio = current.volume as f64 / volume_ma;
        
        if volume_ratio > 1.5 && current.change_percent.unwrap_or(0.0) > 2.0 {
            signals.push(TechnicalSignal {
                symbol: current.date.clone(),
                indicator_name: "放量上涨".to_string(),
                value: volume_ratio,
                signal_strength: 3,
                description: format!("放量{:.1}倍上涨{:.2}%", volume_ratio, current.change_percent.unwrap_or(0.0)),
            });
        }

        Ok(signals)
    }

    /// 计算移动平均线
    fn calculate_moving_averages(&self, prices: &[PriceData]) -> MovingAverages {
        MovingAverages {
            ma5: self.calculate_sma(prices, 5),
            ma10: self.calculate_sma(prices, 10),
            ma20: self.calculate_sma(prices, 20),
            ma60: self.calculate_sma(prices, 60),
            ma120: self.calculate_sma(prices, 120),
            ma250: self.calculate_sma(prices, 250),
        }
    }

    /// 计算简单移动平均
    fn calculate_sma(&self, prices: &[PriceData], period: usize) -> Option<f64> {
        if prices.len() < period {
            return None;
        }

        let sum: f64 = prices.iter()
            .rev()
            .take(period)
            .map(|p| p.close)
            .sum();

        Some(sum / period as f64)
    }

    /// 计算平均成交量
    fn calculate_average_volume(&self, prices: &[PriceData], period: usize) -> f64 {
        if prices.len() < period {
            return prices.iter().map(|p| p.volume as f64).sum::<f64>() / prices.len() as f64;
        }

        let sum: f64 = prices.iter()
            .rev()
            .take(period)
            .map(|p| p.volume as f64)
            .sum();

        sum / period as f64
    }

    /// 保存策略信号到数据库
    pub async fn save_signal(&self, symbol: &str, signal: &StrategySignal) -> Result<()> {
        let (strategy_type, signal_strength, description, key_levels_json) = match signal {
            StrategySignal::LimitUpPullback(s) => (
                "limit_up_pullback",
                s.signal_strength,
                s.description.clone(),
                serde_json::to_string(&s.key_levels)?,
            ),
            StrategySignal::BottomBreakout(s) => (
                "bottom_breakout",
                s.signal_strength,
                s.description.clone(),
                serde_json::to_string(&s.key_levels)?,
            ),
            StrategySignal::TechnicalIndicator(s) => (
                "technical_indicator",
                s.signal_strength,
                s.description.clone(),
                "[]".to_string(),
            ),
        };

        sqlx::query!(
            r#"
            INSERT INTO strategy_signals 
            (symbol, strategy_type, signal_strength, trigger_price, key_levels, description)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            symbol,
            strategy_type,
            signal_strength,
            0.0, // trigger_price - 需要根据具体信号类型设置
            key_levels_json,
            description
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// 获取最新的策略信号
    pub async fn get_recent_signals(&self, limit: i32) -> Result<Vec<(String, StrategySignal)>> {
        let rows = sqlx::query!(
            r#"
            SELECT symbol, strategy_type, signal_strength, description, key_levels, created_at
            FROM strategy_signals 
            ORDER BY created_at DESC 
            LIMIT ?
            "#,
            limit
        )
        .fetch_all(&self.db)
        .await?;

        let mut signals = Vec::new();
        for row in rows {
            // 这里需要根据数据库记录重构信号对象
            // 为简化实现，先返回技术指标信号
            let signal = StrategySignal::TechnicalIndicator(TechnicalSignal {
                symbol: row.symbol.clone(),
                indicator_name: row.strategy_type,
                value: row.signal_strength as f64,
                signal_strength: row.signal_strength as u8,
                description: row.description.unwrap_or_default(),
            });

            signals.push((row.symbol, signal));
        }

        Ok(signals)
    }
}