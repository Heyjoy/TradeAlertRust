@echo off
echo TradeAlertRust 开发环境检查
echo =========================

set /a ErrorCount=0

echo.
echo 基础工具检查:
echo -------------

REM 检查 Rust
rustc --version >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Rust 编译器已安装
    rustc --version
) else (
    echo [ERROR] Rust 编译器未安装
    set /a ErrorCount+=1
)

REM 检查 Cargo
cargo --version >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Cargo 包管理器已安装
    cargo --version
) else (
    echo [ERROR] Cargo 包管理器未安装
    set /a ErrorCount+=1
)

REM 检查 Git
git --version >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Git 版本控制已安装
    git --version
) else (
    echo [ERROR] Git 版本控制未安装
    set /a ErrorCount+=1
)

echo.
echo 项目文件检查:
echo -------------

REM 检查关键文件
set FILES=.cursorrules .cursor\modes.json Cargo.toml src\main.rs docs\AI_CONTEXT.md tasks\current-tasks.md docs\development-status.md

for %%f in (%FILES%) do (
    if exist "%%f" (
        echo [OK] 文件存在: %%f
    ) else (
        echo [ERROR] 文件缺失: %%f
        set /a ErrorCount+=1
    )
)

echo.
echo Cursor 配置检查:
echo ---------------

REM 检查 .cursorrules 内容
if exist ".cursorrules" (
    findstr /C:"TradeAlertRust" .cursorrules >nul 2>&1
    if %errorlevel% == 0 (
        echo [OK] .cursorrules 包含项目规则
    ) else (
        echo [WARN] .cursorrules 可能不包含项目规则
    )
) else (
    echo [ERROR] .cursorrules 文件不存在
    set /a ErrorCount+=1
)

REM 检查 modes.json
if exist ".cursor\modes.json" (
    echo [OK] AI Agent 模式配置存在
) else (
    echo [ERROR] modes.json 文件不存在
    set /a ErrorCount+=1
)

echo.
echo Rust 项目检查:
echo --------------

REM 检查编译
echo 检查项目编译状态...
cargo check --quiet >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] 项目编译无错误
) else (
    echo [ERROR] 项目编译存在错误
    set /a ErrorCount+=1
)

echo.
echo 检查总结:
echo ========

if %ErrorCount% == 0 (
    echo 恭喜！开发环境配置完美！
) else (
    echo 发现 %ErrorCount% 个错误需要修复
)

echo.
echo 错误数量: %ErrorCount%

if %ErrorCount% gtr 0 (
    echo.
    echo 修复建议:
    echo 1. 安装缺失的工具和依赖
    echo 2. 创建缺失的配置文件
    echo 3. 修复编译错误
)

echo.
echo 准备开始 AI 协作开发！

exit /b %ErrorCount% 