@echo off
echo ========================================
echo   TradeAlertRust AI Collaboration
echo   Work Session Startup
echo ========================================

echo.
echo [1/4] Environment Check...
call scripts\dev\development\check_env.bat
if %errorlevel% neq 0 (
    echo ERROR: Environment check failed!
    pause
    exit /b 1
)

echo.
echo [2/4] Git Status Check...
git status --porcelain
if %errorlevel% neq 0 (
    echo WARNING: Git status check failed
)

echo.
echo [3/4] Project Compilation...
cargo check --quiet
if %errorlevel% neq 0 (
    echo ERROR: Project has compilation errors!
    echo Please fix compilation errors before starting work.
    pause
    exit /b 1
)

echo.
echo [4/4] Loading Project Context...
echo - Core documents ready
echo - AI rules loaded
echo - Development environment verified

echo.
echo ========================================
echo   Ready for AI-Powered Development!
echo ========================================
echo.
echo 🤖 AI Context Loading Instructions:
echo ------------------------------------------------
echo AI should now automatically read:
echo 1. docs/START_HERE.md (complete AI collaboration guide)
echo 2. tasks/current-tasks.md (current priorities)
echo 3. docs/dev/development-status.md (project status)
echo 4. Any topic-specific docs based on discussion focus
echo.
echo Available AI Expert Modes:
echo - re  : Rust Expert
echo - ta  : Trading Analyst  
echo - sa  : Security Auditor
echo - do  : DevOps Engineer
echo - ar  : System Architect
echo - dw  : Documentation Writer
echo - qa  : QA Engineer
echo.
echo 📋 Quick Status Summary:
echo - Current Phase: Phase 2 - User Experience Optimization
echo - Recent Achievements: Currency fix, Chinese search, diagnostics
echo - Next Priority: API performance optimization
echo.
echo 🚀 Ready to start! Tell AI your focus area and begin coding.
echo.
pause 