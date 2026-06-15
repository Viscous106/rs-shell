#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut command = String::new();
        io::stdin().read_line(&mut command).uelse if command.starts_with("echo"){
            println!("{}",&command[5..]);
        }nwrap();
        command = command.trim().to_string();
        
        if command.starts_with("type") && &command[5..] == "echo" || &command[5..] == "exit"{
                println!("{} is a shell builtin",&command[5..]);
        }else{
            println!("{}: command not found",command.trim());
        }
    }
}
