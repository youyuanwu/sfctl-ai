use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{ErrorData as McpError, *},
    schemars, tool, tool_handler, tool_router, ServerHandler, ServiceExt, transport::stdio,
};
use serde::Deserialize;
use std::future::Future;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SimpleService {
    tool_router: ToolRouter<SimpleService>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EchoRequest {
    #[schemars(description = "The message to echo back")]
    pub message: String,
}

#[tool_router]
impl SimpleService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Echo back the message sent")]
    async fn echo(
        &self,
        Parameters(request): Parameters<EchoRequest>,
    ) -> Result<CallToolResult, McpError> {
        eprintln!("Received echo request with message: {}", request.message);
        
        let result = CallToolResult::success(vec![Content::text(format!("Echo: {}", request.message))]);
        eprintln!("Sending echo response");
        
        Ok(result)
    }
}

#[tool_handler]
impl ServerHandler for SimpleService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("A simple echo service. Use the echo tool to send messages.".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("Starting simple MCP server...");
    
    let service = SimpleService::new().serve(stdio()).await?;
    eprintln!("Server started, waiting for requests...");
    
    service.waiting().await?;
    
    eprintln!("Server shutting down");
    Ok(())
}

#[tool_router]
impl SimpleService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Echo back the message sent")]
    async fn echo(
        &self,
        Parameters(request): Parameters<EchoRequest>,
    ) -> Result<CallToolResult, McpError> {
        println!("Received echo request with message: {}", request.message);
        Ok(CallToolResult::success(vec![Content::text(format!("Echo: {}", request.message))]))
    }
}

#[tool_handler]
impl ServerHandler for SimpleService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("A simple echo service. Use the echo tool to send messages.".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting simple MCP server...");
    let service = SimpleService::new().serve(stdio()).await?;
    println!("Server started, waiting for requests...");
    service.waiting().await?;
    Ok(())
}