mod mcp_server;

use mcp_server::ServiceFabricServer;
use rmcp::{ServiceExt, transport::stdio};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an instance of our Service Fabric service
    let service = ServiceFabricServer::new().await?.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
