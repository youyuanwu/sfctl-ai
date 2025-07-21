use tokio::io::{self, AsyncBufReadExt, BufReader};

// prompt user to ack an command
pub async fn ack_command(command: &str) -> bool {
    println!("You are about to run the command: {}", command);
    println!("Do you want to proceed? (yes/no)");

    let input = get_user_input().await;

    match input.trim().to_lowercase().as_str() {
        "yes" | "y" => true,
        _ => false,
    }
}

pub async fn get_user_input() -> String {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input = String::new();
    reader
        .read_line(&mut input)
        .await
        .expect("Failed to read line");
    input.trim().to_string()
}
