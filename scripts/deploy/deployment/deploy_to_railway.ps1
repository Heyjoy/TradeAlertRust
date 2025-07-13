#!/usr/bin/env pwsh

Write-Host "🚀 准备Railway部署..." -ForegroundColor Green

# 检查是否在git仓库中
if (-not (Test-Path ".git")) {
    Write-Host "❌ 当前目录不是git仓库" -ForegroundColor Red
    exit 1
}

# 检查git状态
Write-Host "📋 检查git状态..." -ForegroundColor Yellow
git status

# 添加所有更改
Write-Host "📦 添加更改到git..." -ForegroundColor Yellow
git add .

# 提交更改
$commitMessage = "feat: 添加Railway部署支持

- 支持PORT环境变量配置
- 添加Railway配置文件
- 更新服务器配置以支持0.0.0.0绑定
- 添加部署指南"

Write-Host "💾 提交更改..." -ForegroundColor Yellow
git commit -m $commitMessage

# 推送到远程仓库
Write-Host "🚀 推送到GitHub..." -ForegroundColor Yellow
git push

Write-Host "✅ 代码已推送到GitHub！" -ForegroundColor Green
Write-Host ""
Write-Host "🎯 下一步:" -ForegroundColor Cyan
Write-Host "1. 访问 https://railway.app" -ForegroundColor White
Write-Host "2. 创建新项目并连接GitHub仓库" -ForegroundColor White
Write-Host "3. 参考 RAILWAY_DEPLOY_GUIDE.md 配置环境变量" -ForegroundColor White
Write-Host ""
Write-Host "📚 详细步骤请查看: RAILWAY_DEPLOY_GUIDE.md" -ForegroundColor Magenta 