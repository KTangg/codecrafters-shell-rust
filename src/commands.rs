pub mod builtins;
pub mod external;

pub use builtins::Registry;
pub use external::ExtCommand;

use crate::context::ShellContext;

pub trait Command {
    fn name(&self) -> &str;

    fn execute(&self, args: &[String], ctx: &mut ShellContext);
}
