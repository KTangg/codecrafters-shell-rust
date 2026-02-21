use super::BuiltinCommand;
use crate::context::ShellContext;
use std::path::PathBuf;

pub struct Cd;

impl BuiltinCommand for Cd {
    fn name(&self) -> &str {
        "cd"
    }
    fn execute(&self, args: &[String], ctx: &mut ShellContext) {
        if args.len() > 1 {
            println!("{}: too many arguments", self.name());
        }

        let target = match args.first().map(String::as_str) {
            Some("~") => match ctx.env("HOME") {
                Some(home) => PathBuf::from(home),
                None => {
                    eprintln!("{}: HOME not set", self.name());
                    return;
                }
            },
            Some(path) => PathBuf::from(path),
            None => match ctx.env("HOME") {
                Some(home) => PathBuf::from(home),
                None => {
                    eprintln!("{}: HOME not set", self.name());
                    return;
                }
            },
        };

        if let Err(_) = std::env::set_current_dir(&target) {
            eprintln!(
                "{}: {}: No such file or directory",
                self.name(),
                target.to_str().unwrap()
            );
            return;
        }

        match std::env::current_dir() {
            Ok(abs) => ctx.set_cwd(abs),
            Err(_) => {
                eprintln!("{}: failed to get current directory", self.name());
            }
        }
    }
}
