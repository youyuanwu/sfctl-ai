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
        // Remove comments from command
        let command = command
            .lines()
            .filter(|line| !line.trim().starts_with('#'))
            .collect::<Vec<&str>>()
            .join("\n");

        // Use Invoke-Command with a marker to simplify parsing
        let marker = "___COMMAND_END___";
        let wrapped_command = format!(
            "Invoke-Command -ScriptBlock {{ try {{ {} }} catch {{ Write-Output $_.Exception.Message }} }}; Write-Output '{}'\n",
            command, marker
        );

        self.stdin.write_all(wrapped_command.as_bytes()).await?;
        self.stdin.flush().await?;

        let mut output = String::new();
        let mut line = String::new();
        let mut found_marker = false;

        loop {
            line.clear();
            let n = self.stdout.read_line(&mut line).await?;
            if n == 0 {
                break; // EOF
            }
            if line.trim_end() == marker {
                found_marker = true;
                break;
            }
            output.push_str(&line);
        }

        if !found_marker {
            return Ok(output.trim().to_string());
        }

        // Remove the echoed command from the beginning of the output
        // Look for the exact wrapped command that was echoed back
        if let Some(pos) = output.find(wrapped_command.trim_end()) {
            // Remove everything up to and including the echoed command
            output = output[pos + wrapped_command.trim_end().len()..].to_string();
            // Remove any leading newline that might remain
            if output.starts_with('\n') {
                output = output[1..].to_string();
            }
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

        // Test simple command with exact output
        let output = session.run_command("Write-Output 'Hello'").await.unwrap();
        assert_eq!(output, "Hello");

        // Test arithmetic with exact result
        let output = session.run_command("Write-Output (2 + 3)").await.unwrap();
        assert_eq!(output, "5");

        // Test PowerShell process name
        let output = session
            .run_command("Get-Process -Id $PID | Select-Object -ExpandProperty Name")
            .await
            .unwrap();
        assert_eq!(output, "pwsh");

        // Test bad command error handling
        let output = session
            .run_command("Bad-Command-That-Does-Not-Exist")
            .await
            .unwrap();
        assert!(
            output.contains("The term 'Bad-Command-That-Does-Not-Exist' is not recognized"),
            "Output was: {output}",
        );
        assert!(output.contains("Check the spelling of the name"));
    }
}
