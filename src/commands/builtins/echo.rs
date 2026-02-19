use crate::context::ShellContext;

use super::Command;

pub struct Echo;

impl Command for Echo {
    fn name(&self) -> &str {
        "echo"
    }
    fn execute(&self, args: &[String], _ctx: &mut ShellContext) {
        println!("{}", args.join(" "));
    }
}
