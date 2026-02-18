use super::Command;
use crate::context::ShellContext;

use std::{env, path::PathBuf};

pub struct Cd;

impl Command for Cd {
    fn name(&self) -> &str {
        "cd"
    }
    fn execute(&self, args: &[String], ctx: &mut ShellContext) {
        if args.len() > 1 {
            println!("{}: too many arguments", self.name());
        }

        let target = match args.first().map(String::as_str) {
            Some("~") => match ctx.env.get("HOME") {
                Some(home) => PathBuf::from(home),
                None => {
                    eprintln!("{}: HOME not set", self.name());
                    return;
                }
            },
            Some(path) => PathBuf::from(path),
            None => match ctx.env.get("HOME") {
                Some(home) => PathBuf::from(home),
                None => {
                    eprintln!("{}: HOME not set", self.name());
                    return;
                }
            },
        };

        if let Err(e) = env::set_current_dir(&target) {
            eprintln!(
                "{}: {}: No such file or directory",
                self.name(),
                target.to_str().unwrap()
            );
            return;
        }

        ctx.cwd = env::current_dir().unwrap();
    }
}
