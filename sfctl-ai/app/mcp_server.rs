use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ErrorData as McpError, *},
    schemars, tool, tool_handler, tool_router, ServerHandler,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use schemars::JsonSchema;
use std::io::Write;
use std::fs::OpenOptions;
use std::sync::Arc;
use tokio::sync::Mutex;

// Import the pwsh module from the parent crate
use sfctl_ai::pwsh::PwshSession;

// Define a wrapper for tracing that writes to a file instead
fn log_to_file(message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/mcp-server.log") {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}

#[derive(Clone)]
pub struct ServiceFabricServer {
    tool_router: ToolRouter<ServiceFabricServer>,
    pwsh_session: Arc<Mutex<PwshSession>>,
}

impl ServiceFabricServer {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pwsh_session = PwshSession::new()?;
        Ok(Self { 
            tool_router: Self::tool_router(),
            pwsh_session: Arc::new(Mutex::new(pwsh_session)),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServiceFabricCommandParams {
    /// PowerShell command to execute, e.g. "Get-ServiceFabricClusterHealth"
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ServiceFabricConnectParams {
    /// Connection endpoint, e.g. "localhost:19000" for local cluster
    pub endpoint: Option<String>,
}

#[tool_router]
impl ServiceFabricServer {
    #[tool(description = "Connect to a Service Fabric cluster")]
    async fn sf_connect(
        &self,
        Parameters(ServiceFabricConnectParams { endpoint }): Parameters<ServiceFabricConnectParams>,
    ) -> Result<CallToolResult, McpError> {
        let endpoint = endpoint.unwrap_or_else(|| "localhost:19000".to_string());
        log_to_file(&format!("sf_connect called with endpoint: {}", endpoint));
        
        let mut session = self.pwsh_session.lock().await;
        
        // First import the Service Fabric module
        match session.run_command("Import-Module ServiceFabric").await {
            Ok(_) => log_to_file("ServiceFabric module imported successfully"),
            Err(e) => {
                log_to_file(&format!("Failed to import ServiceFabric module: {}", e));
                return Err(McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("Failed to import ServiceFabric module: {}", e)),
                    data: None,
                });
            }
        }
        
        // Connect to the cluster
        let connect_command = format!("Connect-ServiceFabricCluster -ConnectionEndpoint {}", endpoint);
        match session.run_command(&connect_command).await {
            Ok(output) => {
                log_to_file(&format!("Connected to SF cluster: {}", output));
                Ok(CallToolResult::success(vec![Content::text(format!("Connected to Service Fabric cluster at {}\n{}", endpoint, output))]))
            }
            Err(e) => {
                log_to_file(&format!("Failed to connect to SF cluster: {}", e));
                Err(McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("Failed to connect to Service Fabric cluster: {}", e)),
                    data: None,
                })
            }
        }
    }

    #[tool(description = "Execute a Service Fabric PowerShell command")]
    async fn sf_command(
        &self,
        Parameters(ServiceFabricCommandParams { command }): Parameters<ServiceFabricCommandParams>,
    ) -> Result<CallToolResult, McpError> {
        log_to_file(&format!("sf_command called with: {}", command));
        
        let mut session = self.pwsh_session.lock().await;
        
        match session.run_command(&command).await {
            Ok(output) => {
                log_to_file(&format!("SF command executed successfully: {}", command));
                let result = if output.is_empty() {
                    format!("Command '{}' executed successfully (no output)", command)
                } else {
                    output
                };
                Ok(CallToolResult::success(vec![Content::text(result)]))
            }
            Err(e) => {
                log_to_file(&format!("SF command failed: {}", e));
                Err(McpError {
                    code: ErrorCode(-32603),
                    message: Cow::from(format!("PowerShell command failed: {}", e)),
                    data: None,
                })
            }
        }
    }
}

#[tool_handler]
impl ServerHandler for ServiceFabricServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Service Fabric AI Assistant. Use sf_connect to connect to a cluster, then sf_command to execute Service Fabric PowerShell commands like Get-ServiceFabricClusterHealth, Get-ServiceFabricApplication, etc.".to_string()),
        }
    }
}

