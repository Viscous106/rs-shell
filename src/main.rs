#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        
        if comamnd.starts_with("type"){
                println!("{} is a shell builtin",&command[5..]);
        }else{
            println!("{}: command not found",command.trim());
        }
    }
}
