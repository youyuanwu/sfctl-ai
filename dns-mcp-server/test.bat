@echo off
echo Building DNS MCP server and test client...
cargo build --package dns-mcp-server

echo.
echo Running DNS MCP server and client test...
echo This will pipe the client output into the server input and vice versa.
echo.

target\debug\dns-mcp-server.exe < target\debug\client.exe | target\debug\client.exe

echo.
echo Test complete!
echo If you see DNS information above, the server is working correctly.