#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
         print!("$ ");
         io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let mut parts = command.trim().splitn(2, ' ');
        match parts.next() {
            Some("exit") => break,
            Some("echo") => {
                println!("{}", parts.next().unwrap_or(""));
            }
            Some("type") => {
                if let Some(arg) = parts.next() {
                    if ["exit", "echo", "type"].contains(&arg) {
                        println!("{} is a shell builtin", arg);
                    } else {
                        println!("{}: not found", arg);
                    }
                }
            }
            _ => println!("{}: command not found", command.trim())
        }
    }
}
