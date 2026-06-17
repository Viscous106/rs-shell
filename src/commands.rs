use crate::exec::get_executable_path;

/// Handles shell builtin commands if matched, returning true. Otherwise returns false.
pub fn handle_builtin(cmd: &str, args: &[String]) -> bool {
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
            println!("{}", args.join(" "));
            true
        }
        "pwd" => {
            match std::env::current_dir() {
                Ok(path) => {
                    println!("{}", path.display());
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
                    println!("cd: {}: No such file or directory", target_dir);
                }
            }
            true
        }
        "type" => {
            if !args.is_empty() {
                let target = &args[0];
                if is_builtin(target) {
                    println!("{} is a shell builtin", target);
                } else if let Some(path) = get_executable_path(target) {
                    println!("{} is {}", target, path.display());
                } else {
                    println!("{}: not found", target);
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
