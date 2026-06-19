use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use std::process::Stdio;

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
pub fn run_external_command(cmd: &str, args: &[String], stdout_file: Option<(String, bool)>,stderr_file: Option<(String, bool)>) -> Result<std::process::ExitStatus, std::io::Error> {
    let stdout = match stdout_file {
        Some((path, append)) => Stdio::from(
            std::fs::OpenOptions::new()
            .create(true)
            .append(append)
            .write(!append)
            .truncate(!append)
            .open(path)?
        ),
        None => Stdio::inherit(),
    };
    let stderr = match stderr_file {
        Some((path, append)) => Stdio::from(
            std::fs::OpenOptions::new()
            .create(true)
            .append(append)
            .write(!append)
            .truncate(!append)
            .open(path)?
        ),
        None => Stdio::inherit(),
    };
    std::process::Command::new(cmd)
        .args(args)
        .stdout(stdout)
        .stderr(stderr)
        .status()
}

/// pipelines:
pub fn run_pipeline(parts: Vec<Vec<String>>){
    let mut prev_stdout: Option<std::process::ChildStdout> = None;
    let mut prev_builtin_output: Option<Vec<u8>> = None;
    let mut children: Vec<std::process::Child> = Vec::new();
    let last = parts.len() - 1;

    for (i, part) in parts.iter().enumerate() {
        let (args, stdout_redir, _) = crate::parser::parse_redirections(part);
        if args.is_empty() {continue;}
        let cmd = &args[0];
        let cmd_args = &args[1..];

        if crate::commands::is_builtin(cmd){
            if i == last{
                let mut out: Box<dyn std::io::Write> = match stdout_redir{
                    Some((path, append)) => Box::new(
                        std::fs::OpenOptions::new().create(true).append(append).write(!append).truncate(!append).open(path).unwrap()
                    ),
                    None => Box::new(std::io::stdout()),
                };
                crate::commands::handle_builtin(cmd,cmd_args,&mut *out);
            }else{
                let mut buffer = Vec::new();
                crate::commands::handle_builtin(cmd,cmd_args,&mut buffer);
                prev_builtin_output = Some(buffer);
            }
            prev_stdout = None;
            continue;
        }

        let stdin = if let Some(s) = prev_stdout.take(){
            std::process::Stdio::from(s)
        }else if prev_builtin_output.is_some(){
            std::process::Stdio::piped()
        }else{
            std::process::Stdio::inherit()
        };

        let stdout = if i == last {
            match stdout_redir {
                Some((path, append)) => std::process::Stdio::from(
                    std::fs::OpenOptions::new()
                        .create(true)
                        .append(append)
                        .write(!append)
                        .truncate(!append)
                        .open(path)
                        .unwrap()
                ),
                None => std::process::Stdio::inherit(),
            }
        } else {
            std::process::Stdio::piped()
        };

        if let Some(exe_path) = get_executable_path(cmd) {
            let mut child = std::process::Command::new(exe_path)
                .args(cmd_args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn()
                .expect("Failed to spawn pipeline command");
            if let Some(buffer) = prev_builtin_output.take(){
                if let Some(mut child_stdin) = child.stdin.take(){
                    use std::io::Write;
                    child_stdin.write_all(&buffer).unwrap();
                }
            }
            prev_stdout = child.stdout.take();
            children.push(child);
        }else{
            eprintln!("{}: command not found", cmd);
            break;
        }
    }
    for mut child in children{
        let _ = child.wait();
    }
}
