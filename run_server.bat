@echo off
chcp 65001 >nul
title TradeAlert 本地服务

echo.
echo 🚀 启动 TradeAlert 本地服务
echo ============================
echo.

echo 📋 启动中...
cargo run --release

echo.
echo ✅ 服务已启动！
echo 🌐 请在浏览器中打开: http://localhost:8000
echo.
echo 💡 使用说明：
echo - Ctrl+C 停止服务
echo - 浏览器访问 http://localhost:8000
echo - 设置您的邮箱和股票代码
echo.
pause 