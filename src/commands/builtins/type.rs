use super::{BuiltinCommand, ShellContext};
use crate::commands::ExtCommand;

pub struct Type;

impl BuiltinCommand for Type {
    fn name(&self) -> &str {
        "type"
    }
    fn execute(&self, args: &[String], ctx: &mut ShellContext) {
        for arg in args {
            if ctx.registry().check_builtin(arg) {
                println!("{} is a shell builtin", arg)
            } else {
                let Some(path) = ExtCommand::search_binary(arg, ctx) else {
                    return;
                };
                println!("{arg} is {}", path.to_str().unwrap())
            }
        }
    }
}
