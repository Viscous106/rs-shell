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
        
        let _cmd = &args[0];
        let _cmd_args = &args[1..];
        
        let (args, stdout_redirect) = {
            let mut clean = Vec::new();
            let mut file = None;
            let mut i = 0;
            while i < args.len() {
                if (args[i] == ">" || args[i] == "1>") && i + 1 < args.len() {
                    file = Some(args[i+1].clone());
                    i += 2;
                }else{
                    clean.push(args[i].clone());
                    i += 1;
                }
            }
            (clean,file)
        };
        let cmd = &args[0];
        let cmd_args = &args[1..];

        let mut out: Box<dyn Write> = match &stdout_redirect{
            Some(path) => Box::new(std::fs::File::create(path).unwrap()),
            None       => Box::new(io::stdout()),
        };
        if commands::handle_builtin(cmd, cmd_args,&mut *out) {
            continue;
        }
        
        if exec::get_executable_path(cmd).is_some() {
            if let Err(e) = exec::run_external_command(cmd, cmd_args,stdout_redirect) {
                eprintln!("Failed to execute process: {}", e);
            }
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
