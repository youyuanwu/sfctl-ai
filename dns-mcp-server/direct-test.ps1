# A simplified test for our DNS MCP server
# This script sends JSON-RPC requests directly to the server and captures the output

# Path to the server executable
$serverExe = "target\debug\dns-mcp-server.exe"

# Create temporary files for the requests
$initializeFile = "dns-mcp-server\init-request.json"
$lookupFile = "dns-mcp-server\lookup-request.json"

# Create the initialize request
@{
    jsonrpc = "2.0"
    id = 1
    method = "initialize"
    params = @{}
} | ConvertTo-Json -Depth 10 | Out-File -FilePath $initializeFile -Encoding utf8

# Create the DNS lookup request
@{
    jsonrpc = "2.0"
    id = 2
    method = "callTool"
    params = @{
        name = "dns_lookup"
        parameters = @{
            domain = "example.com"
        }
    }
} | ConvertTo-Json -Depth 10 | Out-File -FilePath $lookupFile -Encoding utf8

# Start the server process
Write-Host "Starting DNS MCP server..." -ForegroundColor Green
Write-Host "We will first send an initialize request, then a DNS lookup request" -ForegroundColor Yellow
Write-Host ""

# Run the initialize request
Write-Host "1. Sending initialize request..." -ForegroundColor Cyan
$initResult = Get-Content $initializeFile | & $serverExe
Write-Host "Response:" -ForegroundColor Green
$initResult | ForEach-Object { Write-Host $_ }
Write-Host ""

# Run the DNS lookup request 
Write-Host "2. Sending DNS lookup request for example.com..." -ForegroundColor Cyan
$dnsResult = Get-Content $lookupFile | & $serverExe
Write-Host "Response:" -ForegroundColor Green
$dnsResult | ForEach-Object { Write-Host $_ }

Write-Host ""
Write-Host "Test complete!" -ForegroundColor Green