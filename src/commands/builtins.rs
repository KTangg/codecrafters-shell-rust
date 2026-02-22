use super::ShellContext;
use std::{collections::HashMap, sync::Arc};

// mod cat;
mod cd;
mod echo;
mod exit;
mod pwd;
mod r#type;

pub fn init_registry() -> Registry {
    let mut reg = Registry::new();

    reg.register(Arc::new(echo::Echo));
    reg.register(Arc::new(exit::Exit));
    reg.register(Arc::new(r#type::Type));
    // reg.register(Box::new(cat::Cat));
    reg.register(Arc::new(pwd::Pwd));
    reg.register(Arc::new(cd::Cd));

    reg
}

pub trait BuiltinCommand {
    fn name(&self) -> &str;

    fn execute(&self, args: &[String], ctx: &mut ShellContext);
}

pub struct Registry {
    commands: HashMap<String, Arc<dyn BuiltinCommand>>,
}

impl Registry {
    fn new() -> Registry {
        Registry {
            commands: HashMap::new(),
        }
    }

    fn register(&mut self, command: Arc<dyn BuiltinCommand>) {
        let name = command.name().to_string();

        self.commands.insert(name, command);
    }

    pub fn get_command(&self, name: &str) -> Option<Arc<dyn BuiltinCommand>> {
        self.commands.get(name).cloned()
    }

    pub fn check_builtin(&self, name: &str) -> bool {
        self.commands.get(name).is_some()
    }

    pub fn command_names(&self) -> impl Iterator<Item = &str> {
        self.commands.keys().map(|k| k.as_str())
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
