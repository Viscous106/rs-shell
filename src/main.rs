use std::io::{self, Write};

mod parser;
mod exec;
mod commands;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        
        let command = command.trim_end_matches(|c| c == '\r' || c == '\n');
        let args = parser::parse_arguments(command);
        if args.is_empty() {
            continue;
        }

        let mut parts: Vec<Vec<String>> = Vec::new();
        let mut current: Vec<String> = Vec::new();
        for arg in args {
            if arg == "|" {
                parts.push(current);
                current = Vec::new();
            }else{
                current.push(arg);
            }
        }
        parts.push(current);

        if parts.len() > 1 {
            exec::run_pipeline(parts);
            continue;
        }

        let (mut args, stdout_redirect, stderr_redirect) = parser::parse_redirections(&parts[0]);
        if args.is_empty(){
            continue;
        }
        let background = matches!(args.last().map(|s| s.as_str()), Some("&"));
        if background {
            args.pop();
        }
        if args.is_empty() {
            continue;
        }
        let cmd = &args[0];
        let cmd_args = &args[1..];
        
        if let Some((ref path, append)) = stderr_redirect {
            std::fs::OpenOptions::new()
                .create(true)
                .append(append)
                .write(!append)
                .truncate(!append)
                .open(path)
                .unwrap();
        }
        let mut out: Box<dyn Write> = match &stdout_redirect {
            Some((path, append)) => Box::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(*append)
                    .write(!*append)
                    .truncate(!*append)
                    .open(path)
                    .unwrap()
            ),
            None => Box::new(io::stdout()),
        };

        if commands::handle_builtin(cmd, cmd_args, &mut *out){
            continue;
        }

        if exec::get_executable_path(cmd).is_some(){
            if background {
                match exec::spawn_external_command(cmd, cmd_args, stdout_redirect, stderr_redirect) {
                    Ok(child) => {
                        println!("[1] {}", child.id());
                    }
                    Err(e) => {
                        eprintln!("Falied to execute process: {}", e);
                    }
                }
            } else if let Err(e) = exec::run_external_command(cmd, cmd_args, stdout_redirect, stderr_redirect){
                eprintln!("Falied to execute process: {}", e);
            }
        }else{
            println!("{}: command not found", cmd);
        }
    }
}
