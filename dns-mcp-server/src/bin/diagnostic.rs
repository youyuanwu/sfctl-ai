use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    println!("Starting DNS MCP server diagnostic test");
    
    // Create a process to run the DNS MCP server
    let mut process = Command::new("target\\debug\\dns-mcp-server.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    
    // Get handles to stdin and stdout
    let mut stdin = process.stdin.take().expect("Failed to open stdin");
    let mut stdout = process.stdout.take().expect("Failed to open stdout");
    
    println!("Sending initialize request...");
    
    // Initialize request
    let initialize_request = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    stdin.write_all(initialize_request.as_bytes())?;
    stdin.write_all(b"\n")?;  // Add newline
    stdin.flush()?;
    
    // Read response with timeout
    let mut buffer = [0; 4096];
    println!("Waiting for response...");
    
    // Give some time for the server to respond
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    match stdout.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                println!("Server closed the connection without sending a response.");
            } else {
                let response = String::from_utf8_lossy(&buffer[0..n]);
                println!("Response: {}", response);
            }
        },
        Err(e) => println!("Error reading response: {}", e),
    }
    
    // Clean up
    if let Some(exit_status) = process.try_wait()? {
        println!("Server exited with status: {}", exit_status);
    } else {
        println!("Server is still running, killing it...");
        process.kill()?;
        process.wait()?;
    }
    
    println!("Test complete");
    Ok(())
}