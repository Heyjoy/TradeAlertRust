# Set base URL
$baseUrl = "http://localhost:3000"

# Set headers
$headers = @{
    "Content-Type" = "application/json"
}

Write-Host "`n=== Testing Create Alert ===" -ForegroundColor Green
$createBody = @{
    symbol = "AAPL"
    condition = "Above"
    price = 150.0
} | ConvertTo-Json

try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/alerts" -Method Post -Headers $headers -Body $createBody
    $alert = $response.Content | ConvertFrom-Json
    Write-Host "Create alert successful:" -ForegroundColor Green
    Write-Host ($alert | ConvertTo-Json -Depth 10)
    $alertId = $alert.id
} catch {
    Write-Host "Failed to create alert: $_" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Testing List Alerts ===" -ForegroundColor Green
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/alerts" -Method Get
    $alerts = $response.Content | ConvertFrom-Json
    Write-Host "List alerts successful:" -ForegroundColor Green
    Write-Host ($alerts | ConvertTo-Json -Depth 10)
} catch {
    Write-Host "Failed to list alerts: $_" -ForegroundColor Red
}

Write-Host "`n=== Testing Get Single Alert ===" -ForegroundColor Green
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/alerts/$alertId" -Method Get
    $alert = $response.Content | ConvertFrom-Json
    Write-Host "Get single alert successful:" -ForegroundColor Green
    Write-Host ($alert | ConvertTo-Json -Depth 10)
} catch {
    Write-Host "Failed to get single alert: $_" -ForegroundColor Red
}

Write-Host "`n=== Testing Delete Alert ===" -ForegroundColor Green
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/alerts/$alertId" -Method Delete
    Write-Host "Delete alert successful" -ForegroundColor Green
} catch {
    Write-Host "Failed to delete alert: $_" -ForegroundColor Red
}

Write-Host "`n=== Verifying Delete Result ===" -ForegroundColor Green
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/alerts/$alertId" -Method Get
    Write-Host "Alert still exists, delete might have failed" -ForegroundColor Red
} catch {
    if ($_.Exception.Response.StatusCode -eq 404) {
        Write-Host "Alert successfully deleted" -ForegroundColor Green
    } else {
        Write-Host "Error while verifying delete: $_" -ForegroundColor Red
    }
} 