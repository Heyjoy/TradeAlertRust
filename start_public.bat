@echo off
chcp 65001 >nul
echo 🚀 启动交易预警系统（公网版本）
echo.
echo 正在启动服务器...
set DATABASE_URL=sqlite:trade_alert.db

REM 在后台启动服务器
start /B cargo run --bin trade_alert_rust

REM 等待服务器启动
echo 等待服务器启动...
timeout /t 10 /nobreak >nul

echo.
echo 🌐 启动 ngrok 公网映射...
echo.

REM 启动ngrok
start /B ngrok http 3000

REM 等待ngrok启动
timeout /t 5 /nobreak >nul

echo.
echo 📋 获取公网地址...
echo.

REM 尝试获取ngrok地址
curl -s http://localhost:4040/api/tunnels | findstr "https://.*ngrok-free.app"

echo.
echo ✨ 复制上面的https地址发给朋友！
echo.
echo 🔗 或者打开 http://localhost:4040 查看ngrok管理面板
echo.
echo 按任意键关闭所有服务...
pause >nul

REM 关闭所有相关进程
taskkill /f /im ngrok.exe 2>nul
taskkill /f /im trade_alert_rust.exe 2>nul 