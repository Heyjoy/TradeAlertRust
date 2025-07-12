-- 创建A股股票信息表
CREATE TABLE IF NOT EXISTS cn_stocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL UNIQUE,           -- 股票代码 (如 000001.SZ)
    code TEXT NOT NULL,                    -- 纯代码 (如 000001)
    exchange TEXT NOT NULL,                -- 交易所 (SZ/SH/BJ)
    name_cn TEXT NOT NULL,                 -- 中文名称 (如 平安银行)
    name_en TEXT,                          -- 英文名称 (如 Ping An Bank)
    pinyin TEXT NOT NULL,                  -- 拼音 (如 pinganyhang)
    pinyin_short TEXT NOT NULL,            -- 拼音简写 (如 payh)
    industry TEXT,                         -- 行业
    market_cap REAL,                       -- 市值
    status TEXT DEFAULT 'active',          -- 状态 (active/suspended/delisted)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建搜索索引
CREATE INDEX IF NOT EXISTS idx_cn_stocks_symbol ON cn_stocks(symbol);
CREATE INDEX IF NOT EXISTS idx_cn_stocks_code ON cn_stocks(code);
CREATE INDEX IF NOT EXISTS idx_cn_stocks_name_cn ON cn_stocks(name_cn);
CREATE INDEX IF NOT EXISTS idx_cn_stocks_pinyin ON cn_stocks(pinyin);
CREATE INDEX IF NOT EXISTS idx_cn_stocks_pinyin_short ON cn_stocks(pinyin_short);

-- 插入一些常见A股数据作为示例
INSERT OR IGNORE INTO cn_stocks (symbol, code, exchange, name_cn, name_en, pinyin, pinyin_short, industry) VALUES
('000001.SZ', '000001', 'SZ', '平安银行', 'Ping An Bank', 'pinganyhang', 'payh', '银行'),
('000002.SZ', '000002', 'SZ', '万科A', 'China Vanke', 'wankea', 'wka', '房地产'),
('600036.SH', '600036', 'SH', '招商银行', 'China Merchants Bank', 'zhaoshangyhang', 'zsyh', '银行'),
('600519.SH', '600519', 'SH', '贵州茅台', 'Kweichow Moutai', 'guizhoumoutai', 'gzmt', '白酒'),
('000858.SZ', '000858', 'SZ', '五粮液', 'Wuliangye Yibin', 'wuliangye', 'wly', '白酒'),
('600887.SH', '600887', 'SH', '伊利股份', 'Inner Mongolia Yili', 'yiligufeng', 'ylgf', '食品饮料'),
('000725.SZ', '000725', 'SZ', '京东方A', 'BOE Technology', 'jingdongfanga', 'jdfa', '电子'),
('002415.SZ', '002415', 'SZ', '海康威视', 'Hikvision', 'haikangweishi', 'hkws', '电子'),
('300059.SZ', '300059', 'SZ', '东方财富', 'East Money', 'dongfangcaifu', 'dfcf', '金融服务'),
('002594.SZ', '002594', 'SZ', '比亚迪', 'BYD Company', 'biyadi', 'byd', '汽车');

-- 创建美股常见股票表 (可选)
CREATE TABLE IF NOT EXISTS us_stocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL UNIQUE,           -- 股票代码 (如 AAPL)
    name_en TEXT NOT NULL,                 -- 英文名称 (如 Apple Inc.)
    name_cn TEXT,                          -- 中文名称 (如 苹果公司)
    sector TEXT,                           -- 行业
    market_cap REAL,                       -- 市值
    exchange TEXT,                         -- 交易所 (NASDAQ/NYSE)
    status TEXT DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 插入一些常见美股数据
INSERT OR IGNORE INTO us_stocks (symbol, name_en, name_cn, sector, exchange) VALUES
('AAPL', 'Apple Inc.', '苹果公司', 'Technology', 'NASDAQ'),
('GOOGL', 'Alphabet Inc.', '谷歌', 'Technology', 'NASDAQ'),
('MSFT', 'Microsoft Corporation', '微软', 'Technology', 'NASDAQ'),
('TSLA', 'Tesla Inc.', '特斯拉', 'Automotive', 'NASDAQ'),
('AMZN', 'Amazon.com Inc.', '亚马逊', 'Consumer Discretionary', 'NASDAQ'),
('META', 'Meta Platforms Inc.', 'Meta', 'Technology', 'NASDAQ'),
('NVDA', 'NVIDIA Corporation', '英伟达', 'Technology', 'NASDAQ'),
('NFLX', 'Netflix Inc.', '奈飞', 'Communication Services', 'NASDAQ'),
('PDD', 'PDD Holdings Inc.', '拼多多', 'Consumer Discretionary', 'NASDAQ'),
('BABA', 'Alibaba Group Holding Limited', '阿里巴巴', 'Consumer Discretionary', 'NYSE'); 