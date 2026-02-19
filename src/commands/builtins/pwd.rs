use crate::context::ShellContext;

use super::Command;

pub struct Pwd;

impl Command for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) {
        println!("{}", ctx.cwd.to_str().unwrap());
    }
}
