use rustyline::history;

use crate::commands::{Registry, init_registry};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

pub struct ShellContext {
    registry: Registry,
    env: Env,
    cwd: PathBuf,
    history: History,
}

impl ShellContext {
    pub fn new() -> Self {
        ShellContext {
            registry: init_registry(),
            env: Env::from_system(),
            cwd: env::current_dir().expect("couldn't get cwd"),
            history: History::new(),
        }
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn builtin_names_iter(&self) -> impl Iterator<Item = &str> {
        self.registry.command_names()
    }

    pub fn env(&self, key: &str) -> Option<&str> {
        self.env.get(key)
    }

    pub fn set_env(&mut self, key: &str, value: &str) -> Option<String> {
        self.env.set(key, value)
    }

    pub fn unset_env(&mut self, key: &str) -> Option<String> {
        self.env.unset(key)
    }

    pub fn paths(&self) -> impl Iterator<Item = PathBuf> {
        self.env
            .get("PATH")
            .into_iter()
            .flat_map(|p| p.split(':'))
            .map(PathBuf::from)
    }

    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    pub fn set_cwd(&mut self, path: PathBuf) {
        self.cwd = path;
    }

    pub fn historys(&self) -> impl Iterator<Item = &String> {
        self.history.entries.iter()
    }

    pub fn push_history(&mut self, entity: String) {
        self.history.entries.push(entity.to_string());
    }

    pub fn clear_history(&mut self) {
        self.history.entries.clear();
    }

    pub fn read_history(&mut self, target: &Path) -> io::Result<()> {
        let file = File::options().read(true).open(target)?;
        let buf = BufReader::new(file);

        buf.lines().try_for_each(|line| {
            let line = line?;
            if !line.is_empty() {
                self.push_history(line);
            }
            Ok(())
        })
    }

    pub fn write_history(&self, target: &Path) -> io::Result<()> {
        let file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(target)?;
        let mut writer = BufWriter::new(file);

        for entry in self.historys() {
            writeln!(writer, "{entry}");
        }

        writer.flush()?;
        Ok(())
    }

    pub fn append_history(&self, target: &Path) -> io::Result<()> {
        let file = File::options().create(true).append(true).open(target)?;
        let mut writer = BufWriter::new(file);

        for entry in self.historys() {
            writeln!(writer, "{entry}");
        }

        writer.flush()?;
        Ok(())
    }
}

struct Env {
    vars: HashMap<String, String>,
}

impl Env {
    fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(String::as_str)
    }

    // Replace the old value
    fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.vars.insert(key.to_string(), value.to_string())
    }

    fn unset(&mut self, key: &str) -> Option<String> {
        self.vars.remove(key)
    }

    fn from_system() -> Env {
        Env {
            vars: env::vars().collect(),
        }
    }
}

pub struct History {
    entries: Vec<String>,
}

impl History {
    fn new() -> Self {
        History {
            entries: Vec::new(),
        }
    }
}
