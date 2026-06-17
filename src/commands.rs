use crate::exec::get_executable_path;
use std::io::Write;

/// Handles shell builtin commands if matched, returning true. Otherwise returns false.
pub fn handle_builtin(cmd: &str, args: &[String], output: &mut dyn Write) -> bool {
    match cmd {
        "exit" => {
            let code = if args.is_empty() {
                0
            } else {
                args[0].parse::<i32>().unwrap_or(0)
            };
            std::process::exit(code);
        }
        "echo" => {
            writeln!(output,"{}", args.join(" ")).unwrap();
            true
        }
        "pwd" => {
            match std::env::current_dir() {
                Ok(path) => {
                    writeln!(output,"{}", path.display()).unwrap();
                }
                Err(e) => {
                    eprintln!("pwd: {}", e);
                }
            }
            true
        }
        "cd" => {
            if !args.is_empty() {
                let target_dir = &args[0];
                let resolved_dir = if target_dir == "~" {
                    std::env::var("HOME").ok()
                } else if target_dir.starts_with("~/") {
                    std::env::var("HOME").ok().map(|home| {
                        format!("{}{}", home, &target_dir[1..])
                    })
                } else {
                    None
                };

                let path_to_set = resolved_dir.as_deref().unwrap_or(target_dir);
                if let Err(_) = std::env::set_current_dir(path_to_set) {
                    writeln!(output,"cd: {}: No such file or directory", target_dir).unwrap();
                }
            }
            true
        }
        "type" => {
            if !args.is_empty() {
                let target = &args[0];
                if is_builtin(target) {
                    writeln!(output,"{} is a shell builtin", target).unwrap();
                } else if let Some(path) = get_executable_path(target) {
                    writeln!(output,"{} is {}", target, path.display()).unwrap();
                } else {
                    writeln!(output,"{}: not found", target).unwrap();
                }
            }
            true
        }
        _ => false,
    }
}

/// Helper function to check if a command is a builtin.
pub fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "exit" | "echo" | "pwd" | "cd" | "type")
}
