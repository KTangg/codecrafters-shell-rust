use crate::commands::Registry;
use std::{collections::HashMap, env, path::PathBuf};

pub struct ShellContext<'a> {
    pub registry: &'a Registry,
    pub env: &'a Env,
    pub cwd: PathBuf,
}

pub struct Env {
    vars: HashMap<String, String>,
}

impl Env {
    pub fn get(&self, key: &str) -> Option<&str> {
        let val = self.vars.get(key)?;
        Some(val)
    }

    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.vars.insert(key.to_string(), value.to_string())
    }

    pub fn unset(&mut self, key: &str) -> Option<String> {
        self.vars.remove(key)
    }

    pub fn from_system() -> Env {
        let mut env = Env {
            vars: HashMap::new(),
        };
        for (key, value) in env::vars() {
            env.set(&key, &value);
        }

        env
    }
}
