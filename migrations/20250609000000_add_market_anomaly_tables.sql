-- 创建价格历史表 - 存储历史价格数据，用于技术分析和异常检测
CREATE TABLE IF NOT EXISTS price_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    date DATE NOT NULL,
    open_price REAL NOT NULL,
    high_price REAL NOT NULL,
    low_price REAL NOT NULL,
    close_price REAL NOT NULL,
    volume INTEGER NOT NULL,
    daily_change_percent REAL,
    volume_ratio REAL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建新闻事件表 - 存储新闻事件数据，用于市场情绪分析
CREATE TABLE IF NOT EXISTS news_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    source TEXT NOT NULL,
    sentiment TEXT,
    event_type TEXT NOT NULL,
    published_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建技术指标信号表 - 存储技术指标信号，用于交易决策
CREATE TABLE IF NOT EXISTS technical_signals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    indicator_type TEXT NOT NULL,
    signal_value REAL NOT NULL,
    signal_strength INTEGER,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建市场异动表 - 存储市场异动记录，用于预警和监控
CREATE TABLE IF NOT EXISTS market_anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    anomaly_type TEXT NOT NULL,
    current_price REAL NOT NULL,
    change_percent REAL NOT NULL,
    volume_ratio REAL NOT NULL,
    description TEXT,
    severity INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_price_history_symbol_date ON price_history(symbol, date);
CREATE INDEX IF NOT EXISTS idx_news_events_symbol_published ON news_events(symbol, published_at);
CREATE INDEX IF NOT EXISTS idx_technical_signals_symbol_type ON technical_signals(symbol, indicator_type);
CREATE INDEX IF NOT EXISTS idx_market_anomalies_symbol_type ON market_anomalies(symbol, anomaly_type); 