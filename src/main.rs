#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop {
         print!("$ ");
         io::stdout().flush().unwrap();

        let path_env = std::env::var("PATH").unwrap_or_default();
        let path: Vec<&str> = path_env.split(':').collect();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let mut parts = command.trim().splitn(2, ' ');
        match parts.next() {
            Some("exit") => break,
            Some("echo") => {
                println!("{}", parts.next().unwrap_or(""));
            }
            Some("type") => {
                builtin_type(parts.next().unwrap_or(""), &path);
            }
            _ => println!("{}: command not found", command.trim())
        }
    }
}

fn builtin_type(cmd: &str, path: &Vec<&str>) {
    if ["exit", "echo", "type"].contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else {
        for dir in path {
            let full_path = format!("{}/{}", dir, cmd);
            // Check if the file
            if std::path::Path::new(&full_path).exists() && std::path::Path::new(&full_path).is_file() {
                let metadata = std::fs::metadata(&full_path).unwrap();
                if metadata.permissions().mode() & 0o111 != 0 {
                    println!("{} is {}", cmd, full_path);
                    return;
                }
            }
        }
        println!("{}: not found", cmd);
    }
}
