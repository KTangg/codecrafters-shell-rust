use crate::context::ShellContext;

use super::BuiltinCommand;

pub struct Exit;

impl BuiltinCommand for Exit {
    fn name(&self) -> &str {
        "exit"
    }
    fn execute(&self, _args: &[String], _ctx: &mut ShellContext) {
        std::process::exit(0);
    }
}
