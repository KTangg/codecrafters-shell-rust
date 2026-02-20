use crate::context::ShellContext;
use std::collections::HashMap;

// mod cat;
mod cd;
mod echo;
mod exit;
mod pwd;
mod r#type;

pub fn init_registry() -> Registry {
    let mut reg = Registry::new();

    reg.register(Box::new(echo::Echo));
    reg.register(Box::new(exit::Exit));
    reg.register(Box::new(r#type::Type));
    // reg.register(Box::new(cat::Cat));
    reg.register(Box::new(pwd::Pwd));
    reg.register(Box::new(cd::Cd));

    reg
}

pub trait BuiltinCommand {
    fn name(&self) -> &str;

    fn execute(&self, args: &[String], ctx: &mut ShellContext);
}

pub struct Registry {
    commands: HashMap<String, Box<dyn BuiltinCommand>>,
}

impl Registry {
    fn new() -> Registry {
        Registry {
            commands: HashMap::new(),
        }
    }

    fn register(&mut self, command: Box<dyn BuiltinCommand>) {
        let name = command.name().to_string();

        self.commands.insert(name, command);
    }

    pub fn get_command(&self, name: &str) -> Option<&dyn BuiltinCommand> {
        self.commands.get(name).map(|c| c.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let registry = Registry::new();
        assert!(
            registry.commands.is_empty(),
            "Registry should be empty after creation"
        );
    }

    #[test]
    fn test_register() {}
}
