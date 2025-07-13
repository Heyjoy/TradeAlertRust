@echo off
echo Checking Requirement Documents...
echo.

set count=0
set passed=0

for %%f in (docs\Requirement\*.md) do (
    if not "%%~nf"=="README" if not "%%~nf"=="REQUIREMENT_ID_REGISTRY" if not "%%~nf"=="PHASE1_IMPROVEMENT_SUMMARY" (
        set /a count+=1
        echo Checking: %%~nf.md
        
        findstr /c:"文档信息" "%%f" >nul
        if !errorlevel! equ 0 (
            findstr /c:"功能需求" "%%f" >nul
            if !errorlevel! equ 0 (
                findstr /c:"技术需求" "%%f" >nul
                if !errorlevel! equ 0 (
                    echo   PASS
                    set /a passed+=1
                ) else (
                    echo   FAIL - Missing technical requirements
                )
            ) else (
                echo   FAIL - Missing functional requirements
            )
        ) else (
            echo   FAIL - Missing document info
        )
    )
)

echo.
echo Summary:
echo Total Files: %count%
echo Passed: %passed%
set /a failed=%count%-%passed%
echo Failed: %failed%

if %passed% equ %count% (
    echo All documents comply with standards!
) else (
    echo Some documents need improvement.
) 