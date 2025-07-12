-- 创建加密货币信息表
CREATE TABLE IF NOT EXISTS crypto_stocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL UNIQUE,           -- 交易对代码 (如 BTC-USD)
    base_symbol TEXT NOT NULL,             -- 基础币种 (如 BTC)
    quote_symbol TEXT NOT NULL,            -- 报价币种 (如 USD)
    name_en TEXT NOT NULL,                 -- 英文名称 (如 Bitcoin)
    name_cn TEXT,                          -- 中文名称 (如 比特币)
    category TEXT,                         -- 分类 (如 Layer 1, DeFi, NFT)
    market_cap REAL,                       -- 市值
    circulating_supply REAL,              -- 流通量
    max_supply REAL,                       -- 最大供应量
    status TEXT DEFAULT 'active',          -- 状态 (active/suspended/delisted)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建搜索索引
CREATE INDEX IF NOT EXISTS idx_crypto_stocks_symbol ON crypto_stocks(symbol);
CREATE INDEX IF NOT EXISTS idx_crypto_stocks_base_symbol ON crypto_stocks(base_symbol);
CREATE INDEX IF NOT EXISTS idx_crypto_stocks_name_en ON crypto_stocks(name_en);
CREATE INDEX IF NOT EXISTS idx_crypto_stocks_name_cn ON crypto_stocks(name_cn);

-- 插入主流加密货币数据
INSERT OR IGNORE INTO crypto_stocks (symbol, base_symbol, quote_symbol, name_en, name_cn, category) VALUES
('BTC-USD', 'BTC', 'USD', 'Bitcoin', '比特币', 'Layer 1'),
('ETH-USD', 'ETH', 'USD', 'Ethereum', '以太坊', 'Layer 1'),
('BNB-USD', 'BNB', 'USD', 'BNB', '币安币', 'Exchange'),
('XRP-USD', 'XRP', 'USD', 'XRP', '瑞波币', 'Payment'),
('ADA-USD', 'ADA', 'USD', 'Cardano', '卡尔达诺', 'Layer 1'),
('SOL-USD', 'SOL', 'USD', 'Solana', '索拉纳', 'Layer 1'),
('DOGE-USD', 'DOGE', 'USD', 'Dogecoin', '狗狗币', 'Meme'),
('DOT-USD', 'DOT', 'USD', 'Polkadot', '波卡', 'Layer 0'),
('AVAX-USD', 'AVAX', 'USD', 'Avalanche', '雪崩', 'Layer 1'),
('SHIB-USD', 'SHIB', 'USD', 'Shiba Inu', '柴犬币', 'Meme'),
('LTC-USD', 'LTC', 'USD', 'Litecoin', '莱特币', 'Payment'),
('LINK-USD', 'LINK', 'USD', 'Chainlink', '链环', 'Oracle'),
('UNI-USD', 'UNI', 'USD', 'Uniswap', 'Uniswap', 'DeFi'),
('MATIC-USD', 'MATIC', 'USD', 'Polygon', 'Polygon', 'Layer 2'),
('TRX-USD', 'TRX', 'USD', 'TRON', '波场', 'Layer 1'),

-- 添加一些USDT交易对
('BTC-USDT', 'BTC', 'USDT', 'Bitcoin', '比特币', 'Layer 1'),
('ETH-USDT', 'ETH', 'USDT', 'Ethereum', '以太坊', 'Layer 1'),
('BNB-USDT', 'BNB', 'USDT', 'BNB', '币安币', 'Exchange'),

-- 添加稳定币
('USDT-USD', 'USDT', 'USD', 'Tether', '泰达币', 'Stablecoin'),
('USDC-USD', 'USDC', 'USD', 'USD Coin', 'USD Coin', 'Stablecoin'),
('BUSD-USD', 'BUSD', 'USD', 'Binance USD', 'Binance USD', 'Stablecoin'),
('DAI-USD', 'DAI', 'USD', 'Dai', 'Dai', 'Stablecoin');