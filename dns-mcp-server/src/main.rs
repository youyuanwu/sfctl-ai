use anyhow::Result;
use dns_mcp::DnsService;
use rmcp::{transport::stdio, ServiceExt};

mod dns_mcp;

#[tokio::main]
async fn main() -> Result<()> {
    // Create an instance of our DNS service
    let service = DnsService::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}