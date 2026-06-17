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
        
        let cmd = &args[0];
        let cmd_args = &args[1..];
        
        if commands::handle_builtin(cmd, cmd_args) {
            continue;
        }
        
        if exec::get_executable_path(cmd).is_some() {
            if let Err(e) = exec::run_external_command(cmd, cmd_args) {
                eprintln!("Failed to execute process: {}", e);
            }
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
