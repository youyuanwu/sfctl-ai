# DNS Lookup MCP Server

A simple stdio MCP (Model Context Protocol) server that provides DNS lookup capabilities for AI agents.

## Features

- Perform DNS lookups for any domain
- Uses the HackerTarget API for DNS information
- Compatible with any MCP client (Cursor, Claude Code, etc.)

## Getting Started

### Development

To run the server in development mode:

```bash
cargo run --package dns-mcp-server
```

### Building for Production

To build and install the server for production use:

```bash
cargo install --path dns-mcp-server
```

This installs the `dns-mcp-server` binary to your Cargo bin directory (typically `~/.cargo/bin`), making it available system-wide if this directory is in your PATH.

## Configuration for MCP Clients

### VS Code with Cursor

The `.cursor/mcp.json` file is already configured to enable this MCP server in the Cursor extension for VS Code.

### Global Configuration

To configure the MCP server globally for Cursor, edit the main settings file located at `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "DNS Lookup": {
      "command": "dns-mcp-server"
    }
  }
}
```

## How It Works

The server implements the Model Context Protocol, allowing AI models to:

1. Receive a request to perform a DNS lookup with a domain parameter
2. Use the HackerTarget API to perform the DNS lookup
3. Return the results back to the AI agent

## Example Usage

When the MCP server is running and configured with your AI assistant, you can ask it to perform DNS lookups:

"Can you check the DNS records for example.com, github.com, and google.com and create a table of the results?"

The AI will use the MCP server to fetch real-time DNS information and present it to you.