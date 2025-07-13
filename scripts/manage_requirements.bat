@echo off
setlocal enabledelayedexpansion

rem Requirements Document Management Script
rem Manages local Git versioning without uploading to GitHub

set "REQUIREMENT_PATH=docs\Requirement"
set "ACTION=%1"

if "%ACTION%"=="" set "ACTION=help"

echo Requirements Document Management Tool
echo.

if "%ACTION%"=="help" goto :help
if "%ACTION%"=="status" goto :status
if "%ACTION%"=="commit" goto :commit
if "%ACTION%"=="log" goto :log
if "%ACTION%"=="backup" goto :backup
if "%ACTION%"=="check" goto :check
goto :help

:help
echo Usage: manage_requirements.bat [action]
echo.
echo Available actions:
echo   status   - Check requirements document status
echo   commit   - Commit requirements document changes
echo   log      - Show requirements document commit history
echo   backup   - Backup requirements documents
echo   check    - Check document integrity
echo   help     - Show this help information
echo.
echo Examples:
echo   manage_requirements.bat status
echo   manage_requirements.bat commit
echo   manage_requirements.bat backup
goto :end

:status
echo Requirements Document Status Check
echo.
if not exist "%REQUIREMENT_PATH%" (
    echo ERROR: Requirements directory not found: %REQUIREMENT_PATH%
    goto :end
)

echo Git Status:
git status %REQUIREMENT_PATH% --porcelain

echo.
echo Document Statistics:
for /f %%i in ('dir /b "%REQUIREMENT_PATH%\*.md" 2^>nul ^| find /c /v ""') do (
    echo   Total requirement documents: %%i
)

echo.
git status %REQUIREMENT_PATH% --porcelain >nul 2>&1
if %errorlevel%==0 (
    echo OK: Git status normal
) else (
    echo WARNING: Git status abnormal
)
goto :end

:commit
echo Commit Requirements Document Changes
echo.
set /p "message=Enter commit message: "
if "%message%"=="" (
    echo ERROR: Commit message cannot be empty
    goto :end
)

git add %REQUIREMENT_PATH%
git commit -m "Requirements: %message%"

echo.
echo OK: Requirements documents committed to local Git
echo WARNING: Requirements documents will NOT be pushed to GitHub (protected by .gitignore)
goto :end

:log
echo Requirements Document Commit History
echo.
git log --oneline --graph --decorate %REQUIREMENT_PATH% -10
echo.
echo TIP: Use 'git log %REQUIREMENT_PATH%' to view complete history
goto :end

:backup
echo Backup Requirements Documents
echo.
if not exist "backup" mkdir backup
if not exist "backup\requirements" mkdir backup\requirements

set "timestamp=%date:~0,4%%date:~5,2%%date:~8,2%_%time:~0,2%%time:~3,2%%time:~6,2%"
set "timestamp=%timestamp: =0%"
set "backupfile=backup\requirements\requirements_backup_%timestamp%.zip"

powershell -Command "Compress-Archive -Path '%REQUIREMENT_PATH%\*' -DestinationPath '%backupfile%' -Force"

if exist "%backupfile%" (
    echo OK: Requirements documents backed up to: %backupfile%
) else (
    echo ERROR: Backup failed
)
goto :end

:check
echo Requirements Document Integrity Check
echo.

rem Check required documents
set "missing=0"
if not exist "%REQUIREMENT_PATH%\README.md" (
    echo ERROR: Missing README.md
    set "missing=1"
)
if not exist "%REQUIREMENT_PATH%\1.1-PRD_MASTER.md" (
    echo ERROR: Missing 1.1-PRD_MASTER.md
    set "missing=1"
)
if not exist "%REQUIREMENT_PATH%\REQUIREMENT_ID_REGISTRY.md" (
    echo ERROR: Missing REQUIREMENT_ID_REGISTRY.md
    set "missing=1"
)

if "%missing%"=="0" (
    echo OK: All required documents exist
)

echo.
echo Git Ignore Status Check:
git check-ignore %REQUIREMENT_PATH% >nul 2>&1
if %errorlevel%==0 (
    echo OK: Requirements directory is ignored by Git (will NOT upload to GitHub)
) else (
    echo WARNING: Requirements directory is NOT ignored by Git
    echo          Please check .gitignore file configuration
)
goto :end

:end
echo.
pause 