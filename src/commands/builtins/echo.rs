use super::{BuiltinCommand, ShellContext};
pub struct Echo;

impl BuiltinCommand for Echo {
    fn name(&self) -> &str {
        "echo"
    }
    fn execute(&self, args: &[String], _ctx: &mut ShellContext) {
        println!("{}", args.join(" "));
    }
}
