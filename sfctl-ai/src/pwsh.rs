use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};

pub struct PwshSession {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl PwshSession {
    pub fn new() -> std::io::Result<Self> {
        let mut child = Command::new("pwsh")
            .arg("-NoLogo")
            .arg("-NoProfile")
            .arg("-NonInteractive")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .env("NO_COLOR", "1") // Prevent ANSI color codes
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());

        Ok(PwshSession { stdin, stdout })
    }

    pub async fn run_command(&mut self, command: &str) -> std::io::Result<String> {
        // remove comments in command
        let command = command
            .lines()
            .filter(|line| !line.trim().starts_with('#')) // Remove lines starting with '#'
            .collect::<Vec<&str>>()
            .join("\n");
        let marker = "___END___";
        let full_command = format!("{command}; echo {marker}\n");

        self.stdin.write_all(full_command.as_bytes()).await?;
        self.stdin.flush().await?;

        let mut output = String::new();
        let mut line = String::new();
        loop {
            line.clear();
            let n = self.stdout.read_line(&mut line).await?;
            if n == 0 {
                break; // EOF
            }
            if line.trim_end() == marker {
                break;
            }
            output.push_str(&line);
        }
        // Optionally, remove the echoed command and trailing newlines/prompts
        if let Some(pos) = output.find(&full_command) {
            // Remove the echoed command and everything before it
            output = output[pos + full_command.len()..].to_string();
        }
        Ok(output.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pwsh_session() {
        let mut session = PwshSession::new().unwrap();
        // If add comments, the there is PS heading in the output.
        let output = session
            .run_command("#command \n Get-Process")
            .await
            .unwrap();
        println!("Get-Process Output: {}", output);
        assert!(output.contains("Id"));
        let output = session
            .run_command("Write-Host 'Hello, PowerShell!'")
            .await
            .unwrap();
        assert!(output.contains("Hello, PowerShell!"));

        let output = session.run_command("Bad-command").await.unwrap();
        assert!(output.contains("not recognized"));
        let output = session.run_command("Get-Process -Id $PID").await.unwrap();
        assert!(output.contains("Id"));
        let output = session
            .run_command("Get-Process -Id $PID | Select-Object -ExpandProperty Name")
            .await
            .unwrap();
        assert_eq!(output, "pwsh");
    }
}
