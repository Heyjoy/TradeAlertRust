# Trade Alert Email Test Script
# Usage: .\test_email.ps1

Write-Host "Trade Alert System - Email Test" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan

# Check if server is running
Write-Host "Checking server status..." -ForegroundColor Blue

try {
    $response = Invoke-WebRequest -Uri "http://localhost:3000" -TimeoutSec 5 -ErrorAction Stop
    Write-Host "Server is running" -ForegroundColor Green
} catch {
    Write-Host "Server not running, starting..." -ForegroundColor Yellow
    
    # Check if cargo is available
    try {
        cargo --version | Out-Null
        Write-Host "Starting server, please wait..." -ForegroundColor Blue
        
        # Start server in background
        $job = Start-Job -ScriptBlock {
            Set-Location $using:PWD
            cargo run
        }
        
        # Wait for server to start
        Write-Host "Waiting for server startup..." -ForegroundColor Blue
        Start-Sleep -Seconds 10
        
        # Check again
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:3000" -TimeoutSec 5 -ErrorAction Stop
            Write-Host "Server started successfully" -ForegroundColor Green
        } catch {
            Write-Host "Server startup failed, please run 'cargo run' manually" -ForegroundColor Red
            Stop-Job $job -Force
            Remove-Job $job -Force
            exit 1
        }
    } catch {
        Write-Host "Cargo not found, please install Rust and run 'cargo run'" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "Sending test email..." -ForegroundColor Blue

try {
    $response = Invoke-RestMethod -Uri "http://localhost:3000/api/test-email" -Method Get -ErrorAction Stop
    
    if ($response.success -eq $true) {
        Write-Host "Test email sent successfully!" -ForegroundColor Green
        Write-Host "$($response.message)" -ForegroundColor Green
        Write-Host ""
        Write-Host "Please check your email inbox (including spam folder)" -ForegroundColor Yellow
        Write-Host "If you didn't receive the email, please check:" -ForegroundColor Yellow
        Write-Host "  1. Email configuration in config file or environment variables" -ForegroundColor Yellow
        Write-Host "  2. Email password should be app-specific password" -ForegroundColor Yellow
        Write-Host "  3. Network connection" -ForegroundColor Yellow
    } else {
        Write-Host "Test email failed!" -ForegroundColor Red
        Write-Host "$($response.message)" -ForegroundColor Red
        Write-Host ""
        Write-Host "Common solutions:" -ForegroundColor Yellow
        Write-Host "  1. Check email configuration" -ForegroundColor Yellow
        Write-Host "  2. Use app-specific password (not account password)" -ForegroundColor Yellow
        Write-Host "  3. Check firewall and network settings" -ForegroundColor Yellow
        Write-Host "  4. Check server logs for detailed error information" -ForegroundColor Yellow
    }
} catch {
    Write-Host "Request failed: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Please ensure server is running and check network connection" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Test completed!" -ForegroundColor Cyan

# Ask if user wants to view logs
$viewLogs = Read-Host "View server logs? (y/N)"
if ($viewLogs -eq "y" -or $viewLogs -eq "Y") {
    Write-Host "Please check the terminal window running 'cargo run' for detailed logs" -ForegroundColor Blue
}

Write-Host ""
Write-Host "Done!" -ForegroundColor Green 