#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

fn get_executable_path(cmd: &str) -> Option<PathBuf> {
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

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        if command.is_empty(){
            continue;
        }
        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];
        
        match cmd{
            "exit" => {
                let code = if args.is_empty(){
                    0
                }else {
    	            args[0].parse::<i32>().unwrap_or(0)
                };
                std::process::exit(code);
            }
            "echo" => {
                println!("{}",args.join(" "));
            }
            "type" => {
                if args.is_empty(){
                }else{
                    let target = args[0];
                    if target == "echo" || target == "exit" || target == "type"{
                        println!("{} is a shell builtin",target);
                    }else if let Some(path) = get_executable_path(target)
                        println!("{} is {}",target,path.display());//this is the path of the file
                    }else{
                        println!("{}: not found",target);
                    }
                }
            }
        _ => {
            if get_executable_path(cmd).is_some(){
                if let Err(e) = std::process::Command:new(cmd).args(args).status(){
                    eprintln!("Failed to execute process: {}",e);
                }
            }else{
                println!("{}: command not found", cmd);
            }
        }
    }
}
