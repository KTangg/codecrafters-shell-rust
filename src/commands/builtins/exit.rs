use crate::context::ShellContext;

use super::Command;

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &str {
        "exit"
    }
    fn execute(&self, _args: &[String], _ctx: &mut ShellContext) {
        std::process::exit(0);
    }
}
