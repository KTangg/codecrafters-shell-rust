use crate::context::ShellContext;

use super::BuiltinCommand;

pub struct Pwd;

impl BuiltinCommand for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) {
        println!("{}", ctx.cwd.to_str().unwrap());
    }
}
