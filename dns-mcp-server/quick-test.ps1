Write-Host "Sending a test request to the DNS MCP server..." -ForegroundColor Green
Write-Host "Looking up DNS information for example.com" -ForegroundColor Yellow

# Create a temp file for the test request
$tempFile = New-TemporaryFile
$requestFile = "$tempFile.json"
Rename-Item -Path $tempFile -NewName $requestFile

# Initialize request
@{
    jsonrpc = "2.0"
    id = 1
    method = "initialize"
    params = @{}
} | ConvertTo-Json -Depth 10 | Out-File -FilePath $requestFile -Encoding utf8

# Send initialize request to the server
Get-Content $requestFile | cargo run --package dns-mcp-server

# Prepare callTool request
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
} | ConvertTo-Json -Depth 10 | Out-File -FilePath $requestFile -Force -Encoding utf8

Write-Host "`nSending DNS lookup request for example.com..." -ForegroundColor Cyan
Write-Host "This should return DNS information if the server is working correctly." -ForegroundColor Cyan

# Send callTool request to the server
Get-Content $requestFile | cargo run --package dns-mcp-server

# Clean up
Remove-Item $requestFile -Force