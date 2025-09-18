Write-Host "Building DNS MCP Server..." -ForegroundColor Green
cargo build --package dns-mcp-server

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error building the server." -ForegroundColor Red
    exit 1
}

Write-Host "`nStarting DNS MCP Server..." -ForegroundColor Green
Write-Host "This script will run the DNS MCP server in the background." -ForegroundColor Yellow
Write-Host "You can use it with your MCP client (like Cursor or Claude Code).`n" -ForegroundColor Yellow

# Create a simple test
$testDir = New-Item -Path ".\dns-mcp-server\test" -ItemType Directory -Force
$testFile = Join-Path $testDir "test-dns.json"

# JSON-RPC request for callTool method with dns_lookup and domain=example.com
$callToolRequest = @{
    jsonrpc = "2.0"
    id = 1
    method = "callTool"
    params = @{
        name = "dns_lookup"
        parameters = @{
            domain = "example.com"
        }
    }
} | ConvertTo-Json -Depth 10

# Save the test request
$callToolRequest | Out-File -FilePath $testFile -Encoding utf8

Write-Host "To manually test the server, run the following commands:" -ForegroundColor Cyan
Write-Host "1. In one terminal window, start the server:" -ForegroundColor Cyan
Write-Host "   cargo run --package dns-mcp-server" -ForegroundColor Yellow
Write-Host "2. In another terminal window, send a test request:" -ForegroundColor Cyan
Write-Host "   Get-Content .\dns-mcp-server\test\test-dns.json | cargo run --package dns-mcp-server" -ForegroundColor Yellow

Write-Host "`nAlternatively, you can use the MCP server with Cursor or Claude Code." -ForegroundColor Green
Write-Host "The configuration in .cursor/mcp.json is already set up for this." -ForegroundColor Green