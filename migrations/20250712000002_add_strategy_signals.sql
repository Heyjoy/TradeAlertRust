-- 创建策略信号表
CREATE TABLE IF NOT EXISTS strategy_signals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    strategy_type TEXT NOT NULL,        -- 'limit_up_pullback', 'bottom_breakout', 'technical_indicator'
    signal_strength INTEGER NOT NULL,   -- 1-5 信号强度
    trigger_price REAL NOT NULL,        -- 触发价格
    key_levels TEXT,                    -- JSON: 关键价位信息 [支撑位, 阻力位, 目标位]
    description TEXT,                   -- 信号描述
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,                -- 信号有效期（可选）
    is_active BOOLEAN DEFAULT 1         -- 是否活跃
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_strategy_signals_symbol_type ON strategy_signals(symbol, strategy_type);
CREATE INDEX IF NOT EXISTS idx_strategy_signals_created ON strategy_signals(created_at);
CREATE INDEX IF NOT EXISTS idx_strategy_signals_active ON strategy_signals(is_active);
CREATE INDEX IF NOT EXISTS idx_strategy_signals_strength ON strategy_signals(signal_strength);

-- 创建策略分析结果汇总表
CREATE TABLE IF NOT EXISTS strategy_analysis (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    analysis_date DATE NOT NULL,
    total_signals INTEGER DEFAULT 0,
    avg_signal_strength REAL DEFAULT 0,
    composite_score REAL DEFAULT 0,     -- 综合评分 0-100
    risk_level TEXT,                    -- 'low', 'medium', 'high'
    recommendation TEXT,                -- 'buy', 'hold', 'sell', 'watch'
    key_points TEXT,                    -- JSON: 关键分析要点
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    -- 确保每个股票每天只有一条分析记录
    UNIQUE(symbol, analysis_date)
);

-- 创建策略分析汇总表索引
CREATE INDEX IF NOT EXISTS idx_strategy_analysis_symbol_date ON strategy_analysis(symbol, analysis_date);
CREATE INDEX IF NOT EXISTS idx_strategy_analysis_score ON strategy_analysis(composite_score);
CREATE INDEX IF NOT EXISTS idx_strategy_analysis_recommendation ON strategy_analysis(recommendation);