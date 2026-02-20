use crate::commands::ExtCommand;
use crate::context::ShellContext;

use super::BuiltinCommand;

pub struct Type;

impl BuiltinCommand for Type {
    fn name(&self) -> &str {
        "type"
    }
    fn execute(&self, args: &[String], ctx: &mut ShellContext) {
        for arg in args {
            match ctx.registry.get_command(arg) {
                Some(cmd) => {
                    println!("{} is a shell builtin", cmd.name())
                }
                _ => {
                    let Some(path) = ExtCommand::search_binary(arg, ctx) else {
                        return;
                    };
                    println!("{arg} is {}", path.to_str().unwrap())
                }
            }
        }
    }
}
