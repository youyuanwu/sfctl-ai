#[derive(Debug, PartialEq, Eq)]
pub enum CmdKind {
    Read,
    Write,
    Unknown,
}

/// Simple heuristic to classify PowerShell commands into Read, Write, or Unknown
pub fn classify_cmd(cmd: &str) -> CmdKind {
    let cmd = cmd.trim_start();

    // Special cases
    if cmd.starts_with("Import-Module") {
        return CmdKind::Read;
    }

    if cmd.contains("|") {
        CmdKind::Unknown // Don't classify piped commands
    } else if cmd.starts_with("Get-") || cmd.starts_with("Select-") || cmd.starts_with("Read-") {
        CmdKind::Read
    } else if cmd.starts_with("Set-")
        || cmd.starts_with("New-")
        || cmd.starts_with("Add-")
        || cmd.starts_with("Remove-")
        || cmd.starts_with("Update-")
        || cmd.starts_with("Write-")
    {
        CmdKind::Write
    } else {
        CmdKind::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_cmd() {
        assert_eq!(classify_cmd("Import-Module ServiceFabric"), CmdKind::Read);
        assert_eq!(classify_cmd("Restart-ServiceFabricNode"), CmdKind::Unknown);
        assert_eq!(
            classify_cmd("Connect-ServiceFabricCluster"),
            CmdKind::Unknown
        );
    }
}
