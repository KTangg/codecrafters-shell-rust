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
                builtin_type("type", parts.next().unwrap_or(""), &path);
            }
            Some("pwd") => {
                let _ = builtin_pwd();
            }
            Some(cmd) => {
                exec_path_executer(cmd, parts.next().unwrap_or(""), &path);
            }
            _ => { /* TODO Exit shell on empty */ }
        }
    }
}

// Wrapper for path finding and execute given function
fn path_executer<F>(cmd: &str, arg: &str, path: &Vec<&str>, func: F)
where
    F: Fn(&str, &str, &str),
{
    for dir in path {
        let full_path = format!("{}/{}", dir, cmd);
        if std::path::Path::new(&full_path).exists() && std::path::Path::new(&full_path).is_file() {
            let metadata = std::fs::metadata(&full_path).unwrap();
            if metadata.permissions().mode() & 0o111 != 0 {
                func(cmd, arg, &full_path);
                return;
            }
        }
    }
    println!("{}: not found", cmd);
}

fn builtin_pwd() -> std::io::Result<()> {
    let cwd = std::env::current_dir()?;
    println!("{}", cwd.display());

    Ok(())
}

fn builtin_type(_cmd: &str, args: &str, path: &Vec<&str>) {
    for arg in args.split_whitespace() {
        if ["exit", "echo", "type"].contains(&arg) {
            println!("{} is a shell builtin", arg);
        } else {
            fn type_func(cmd: &str, _arg: &str, full_path: &str) {
                println!("{} is {}", cmd, full_path);
            }
            path_executer(arg, "", path, type_func);
        }
    }
}

fn exec_path_executer(cmd: &str, args: &str, path: &Vec<&str>) {
    fn exec_func(cmd: &str, arg: &str, _full_path: &str) {
        let mut child = std::process::Command::new(cmd)
            .args(arg.split_whitespace())
            .spawn()
            .expect("Failed to execute command");

        child.wait().expect("Failed to wait on child");
    }
    path_executer(cmd, args, path, exec_func);
}
