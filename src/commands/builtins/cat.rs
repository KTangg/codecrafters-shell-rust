use super::Command;
use crate::context::ShellContext;
use std::{fmt::Debug, fs, io::ErrorKind};

pub struct Cat;

impl Command for Cat {
    fn name(&self) -> &str {
        "cat"
    }
    fn execute(&self, args: &[String], ctx: &mut ShellContext) {
        for arg in args {
            match fs::read_to_string(arg) {
                Ok(content) => {
                    print!("{content}");
                }
                Err(e) => {
                    let msg = match e.kind() {
                        ErrorKind::NotFound => "No such file or directory",
                        ErrorKind::PermissionDenied => "Permission denied",
                        _ => "Error",
                    };

                    eprintln!("{}: {}: {}", self.name(), arg, msg);
                }
            }
        }
    }
}
