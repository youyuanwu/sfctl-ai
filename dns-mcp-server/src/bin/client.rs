use std::io::{self, Read, Write};
use serde_json::{json, Value};

fn main() -> io::Result<()> {
    // Initialize request
    let initialize_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    });

    // Write initialize request to the MCP server
    let initialize_str = initialize_request.to_string();
    println!("Sending initialize request: {}", initialize_str);
    io::stdout().write_all(initialize_str.as_bytes())?;
    io::stdout().flush()?;

    // Read response
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("Received response: {}", buffer);

    // Parse response
    let response: Value = serde_json::from_str(&buffer)?;
    
    // Check if initialization was successful
    if response["result"].is_object() {
        println!("Initialization successful!");
        
        // Call DNS lookup tool
        let call_tool_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "callTool",
            "params": {
                "name": "dns_lookup",
                "parameters": {
                    "domain": "example.com"
                }
            }
        });

        // Write callTool request to the MCP server
        let call_tool_str = call_tool_request.to_string();
        println!("Sending callTool request: {}", call_tool_str);
        io::stdout().write_all(call_tool_str.as_bytes())?;
        io::stdout().flush()?;

        // Read response
        let mut tool_buffer = String::new();
        io::stdin().read_to_string(&mut tool_buffer)?;
        println!("Received tool response: {}", tool_buffer);
    } else {
        println!("Initialization failed: {:?}", response);
    }

    Ok(())
}