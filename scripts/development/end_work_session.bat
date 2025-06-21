@echo off
echo ========================================
echo   TradeAlertRust AI Collaboration
echo   Work Session Wrap-up
echo ========================================

echo.
echo [1/5] Final Code Quality Check...
cargo check --quiet
if %errorlevel% neq 0 (
    echo WARNING: Project has compilation errors!
    echo Please fix before committing.
)

cargo clippy --quiet
if %errorlevel% neq 0 (
    echo WARNING: Clippy found issues!
)

echo.
echo [2/5] Running Tests...
cargo test --quiet
if %errorlevel% neq 0 (
    echo WARNING: Some tests failed!
)

echo.
echo [3/5] Code Formatting...
cargo fmt
echo Code formatted successfully.

echo.
echo [4/5] Git Status Summary...
echo Current changes:
git status --short

echo.
echo [5/5] Work Session Summary...
echo.
echo Please update the following files:
echo - tasks/current-tasks.md (mark completed tasks)
echo - docs/development-status.md (update progress)
echo.
echo Recommended next steps:
echo 1. Review and commit your changes
echo 2. Update documentation if needed
echo 3. Plan tomorrow's work priorities
echo.
echo ========================================
echo   Work Session Complete!
echo ========================================
echo.
echo Don't forget to:
echo [ ] Commit your changes with clear messages
echo [ ] Update task tracking documents  
echo [ ] Push to remote repository
echo [ ] Plan next session priorities
echo.
pause 