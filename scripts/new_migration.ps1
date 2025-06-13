# 创建新的数据库迁移文件
# 使用方法: .\scripts\new_migration.ps1 "migration_name"

param(
    [Parameter(Mandatory=$true)]
    [string]$MigrationName
)

Write-Host "📝 创建新的迁移文件..." -ForegroundColor Green

# 设置环境变量
$env:DATABASE_URL = "sqlite:data/trade_alert.db"

# 生成时间戳
$timestamp = Get-Date -Format "yyyyMMddHHmmss"
$filename = "${timestamp}_${MigrationName}.sql"
$filepath = "migrations/$filename"

# 确保 migrations 目录存在
if (!(Test-Path "migrations")) {
    New-Item -ItemType Directory -Path "migrations"
    Write-Host "📁 创建迁移目录" -ForegroundColor Yellow
}

# 创建迁移文件模板
$template = @"
-- Migration: $MigrationName
-- Created: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

-- Add your SQL statements here
-- Example:
-- CREATE TABLE IF NOT EXISTS example_table (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     name TEXT NOT NULL,
--     created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
-- );

-- CREATE INDEX IF NOT EXISTS idx_example_table_name ON example_table(name);
"@

# 写入文件
$template | Out-File -FilePath $filepath -Encoding UTF8

Write-Host "✅ 迁移文件已创建: $filepath" -ForegroundColor Green
Write-Host "📝 请编辑文件添加你的 SQL 语句" -ForegroundColor Cyan
Write-Host "🔄 完成后运行: .\scripts\dev_migrate.ps1" -ForegroundColor Cyan

# 可选：打开文件编辑
$openFile = Read-Host "是否现在打开文件编辑? (y/N)"
if ($openFile -eq "y" -or $openFile -eq "Y") {
    if (Get-Command "code" -ErrorAction SilentlyContinue) {
        code $filepath
    } else {
        notepad $filepath
    }
} 