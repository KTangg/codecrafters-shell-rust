use std::path::PathBuf;

use super::{BuiltinCommand, ShellContext};

pub struct Exit;

impl BuiltinCommand for Exit {
    fn name(&self) -> &str {
        "exit"
    }
    fn execute(&self, _args: &[String], ctx: &mut ShellContext) {
        if let Some(target) = ctx.env("HISTFILE") {
            let _ = ctx.flush_history(&PathBuf::from(target));
        }
        std::process::exit(0);
    }
}
