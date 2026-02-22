use std::path::PathBuf;

use super::{BuiltinCommand, ShellContext};

pub struct Exit;

impl BuiltinCommand for Exit {
    fn name(&self) -> &str {
        "exit"
    }
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) {
        ctx.shutdown();
        std::process::exit(0);
    }
}
