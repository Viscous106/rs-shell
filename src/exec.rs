use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

/// Checks if the command is executable and returns its path.
/// Handles relative/absolute paths directly if they contain a slash `/`.
pub fn get_executable_path(cmd: &str) -> Option<PathBuf> {
    if cmd.contains('/') {
        let path = PathBuf::from(cmd);
        if let Ok(metadata) = std::fs::metadata(&path) {
            if metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
                return Some(path);
            }
        }
        return None;
    }

    if let Ok(path_env) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_env) {
            let full_path = dir.join(cmd);
            if let Ok(metadata) = std::fs::metadata(&full_path) {
                if metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
                    return Some(full_path);
                }
            }
        }
    }
    None
}

/// Executes an external command.
pub fn run_external_command(cmd: &str, args: &[String]) -> Result<std::process::ExitStatus, std::io::Error> {
    std::process::Command::new(cmd)
        .args(args)
        .status()
}
