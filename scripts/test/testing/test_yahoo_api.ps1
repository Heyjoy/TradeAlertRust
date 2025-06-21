# Test Yahoo Finance API
Write-Host "=== Testing Yahoo Finance API ===" -ForegroundColor Green

# Test different stock symbols
$symbols = @("AAPL", "GOOGL", "MSFT", "TSLA")

foreach ($symbol in $symbols) {
    Write-Host "`nTesting symbol: $symbol" -ForegroundColor Yellow
    
    try {
        $url = "https://query1.finance.yahoo.com/v8/finance/chart/$symbol"
        $headers = @{
            "User-Agent" = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        }
        
        $response = Invoke-RestMethod -Uri $url -Headers $headers -TimeoutSec 10
        
        if ($response.chart.result -and $response.chart.result.Count -gt 0) {
            $result = $response.chart.result[0]
            $price = $result.meta.regularMarketPrice
            $volume = $result.meta.regularMarketVolume
            
            Write-Host "✅ Success!" -ForegroundColor Green
            Write-Host "   Price: $price" -ForegroundColor Cyan
            Write-Host "   Volume: $volume" -ForegroundColor Cyan
        } else {
            Write-Host "❌ No data returned" -ForegroundColor Red
        }
    }
    catch {
        Write-Host "❌ Error: $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`n=== Testing Trade Alert API ===" -ForegroundColor Green

# Test creating an alert with a real stock
try {
    $baseUrl = "http://localhost:3000"
    $alertData = @{
        symbol = "AAPL"
        condition = "Above"
        price = 150.0
    } | ConvertTo-Json

    $headers = @{
        "Content-Type" = "application/json"
    }

    Write-Host "Creating test alert for AAPL..." -ForegroundColor Yellow
    $response = Invoke-RestMethod -Uri "$baseUrl/api/alerts" -Method POST -Headers $headers -Body $alertData
    
    Write-Host "✅ Alert created successfully!" -ForegroundColor Green
    Write-Host "Alert ID: $($response.id)" -ForegroundColor Cyan
    
    # Wait a bit and check if price was fetched
    Write-Host "`nWaiting 5 seconds for price update..." -ForegroundColor Yellow
    Start-Sleep -Seconds 5
    
    $priceResponse = Invoke-RestMethod -Uri "$baseUrl/api/prices/AAPL/latest"
    Write-Host "✅ Latest price fetched!" -ForegroundColor Green
    Write-Host "Price: $($priceResponse.price)" -ForegroundColor Cyan
    Write-Host "Volume: $($priceResponse.volume)" -ForegroundColor Cyan
    Write-Host "Timestamp: $($priceResponse.timestamp)" -ForegroundColor Cyan
    
} catch {
    Write-Host "❌ Error testing Trade Alert API: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n=== Test Complete ===" -ForegroundColor Green 