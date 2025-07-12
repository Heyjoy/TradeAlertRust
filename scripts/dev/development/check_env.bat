@echo off
echo TradeAlertRust Development Environment Check
echo ==========================================

set /a ErrorCount=0

echo.
echo Checking basic tools:
echo --------------------

REM Check Rust
rustc --version >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Rust compiler installed
    rustc --version
) else (
    echo [ERROR] Rust compiler not found
    set /a ErrorCount+=1
)

REM Check Cargo
cargo --version >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Cargo package manager installed
    cargo --version
) else (
    echo [ERROR] Cargo package manager not found
    set /a ErrorCount+=1
)

REM Check Git
git --version >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Git version control installed
    git --version
) else (
    echo [ERROR] Git version control not found
    set /a ErrorCount+=1
)

echo.
echo Checking project files:
echo ----------------------

REM Check key files
if exist ".cursorrules" (
    echo [OK] .cursorrules exists
) else (
    echo [ERROR] .cursorrules missing
    set /a ErrorCount+=1
)

if exist ".cursor/modes.json" (
    echo [OK] .cursor/modes.json exists
) else (
    echo [ERROR] .cursor/modes.json missing
    set /a ErrorCount+=1
)

if exist "Cargo.toml" (
    echo [OK] Cargo.toml exists
) else (
    echo [ERROR] Cargo.toml missing
    set /a ErrorCount+=1
)

if exist "src\main.rs" (
    echo [OK] src/main.rs exists
) else (
    echo [ERROR] src/main.rs missing
    set /a ErrorCount+=1
)

if exist "AI_CONTEXT.md" (
    echo [OK] AI_CONTEXT.md exists
) else (
    echo [ERROR] AI_CONTEXT.md missing
    set /a ErrorCount+=1
)

if exist "tasks\current-tasks.md" (
    echo [OK] tasks/current-tasks.md exists
) else (
    echo [ERROR] tasks/current-tasks.md missing
    set /a ErrorCount+=1
)

if exist "docs\dev\development-status.md" (
    echo [OK] docs/dev/development-status.md exists
) else (
    echo [ERROR] docs/dev/development-status.md missing
    set /a ErrorCount+=1
)

echo.
echo Checking Cursor configuration:
echo -----------------------------

REM Check .cursorrules content
if exist ".cursorrules" (
    findstr /C:"TradeAlertRust" .cursorrules >nul 2>&1
    if %errorlevel% == 0 (
        echo [OK] .cursorrules contains project rules
    ) else (
        echo [WARN] .cursorrules may not contain project rules
    )
) else (
    echo [ERROR] .cursorrules file not found
    set /a ErrorCount+=1
)

echo.
echo Checking Rust project:
echo ---------------------

echo Checking project compilation...
cargo check --quiet >nul 2>&1
if %errorlevel% == 0 (
    echo [OK] Project compiles without errors
) else (
    echo [ERROR] Project has compilation errors
    set /a ErrorCount+=1
)

echo.
echo Summary:
echo ========

if %ErrorCount% == 0 (
    echo Congratulations! Development environment is perfect!
) else (
    echo Found %ErrorCount% errors that need to be fixed
)

echo.
echo Error count: %ErrorCount%

if %ErrorCount% gtr 0 (
    echo.
    echo Suggestions:
    echo 1. Install missing tools and dependencies
    echo 2. Create missing configuration files
    echo 3. Fix compilation errors
)

echo.
echo Ready to start AI-powered development!

exit /b %ErrorCount% 