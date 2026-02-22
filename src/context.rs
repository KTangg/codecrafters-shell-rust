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

    pub fn set_cwd(&mut self, path: &Path) {
        self.cwd = PathBuf::from(path);
    }

    pub fn historys(&self) -> &[String] {
        &self.history.entries
    }

    pub fn push_history(&mut self, entry: &str) {
        self.history.push(entry);
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn preload_history(&mut self, target: &Path) -> io::Result<()> {
        self.history.read(target)?;
        self.history.mark_flushed();

        Ok(())
    }

    pub fn read_history(&mut self, target: &Path) -> io::Result<()> {
        self.history.read(target)?;

        Ok(())
    }

    pub fn write_history(&self, target: &Path) -> io::Result<()> {
        self.history.write(target)?;

        Ok(())
    }

    pub fn flush_history(&mut self, target: &Path) -> io::Result<()> {
        self.history.flush(target)?;

        Ok(())
    }

    pub fn shutdown(&mut self) {
        // Gracefully exit shell
        if let Some(path) = self.env("HISTFILE") {
            let _ = self.flush_history(&PathBuf::from(path));
        }
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
    flush_index: usize,
}

impl History {
    fn new() -> Self {
        History {
            entries: Vec::new(),
            flush_index: 0,
        }
    }

    fn push(&mut self, entry: &str) {
        self.entries.push(entry.to_string());
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.flush_index = 0;
    }

    fn mark_flushed(&mut self) {
        self.flush_index = self.entries.len();
    }

    fn unflushed_history(&self) -> &[String] {
        self.entries.get(self.flush_index..).unwrap_or(&[])
    }

    fn read(&mut self, target: &Path) -> io::Result<()> {
        let file = File::options().read(true).open(target)?;
        let buf = BufReader::new(file);

        buf.lines().try_for_each(|line| {
            let line = line?;
            if !line.is_empty() {
                self.push(&line);
            }
            Ok(())
        })
    }

    fn write(&self, target: &Path) -> io::Result<()> {
        let file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(target)?;
        let mut writer = BufWriter::new(file);

        for entry in self.entries.iter() {
            writeln!(writer, "{entry}")?;
        }

        writer.flush()?;
        Ok(())
    }

    fn flush(&mut self, target: &Path) -> io::Result<()> {
        let file = File::options().create(true).append(true).open(target)?;
        let mut writer = BufWriter::new(file);

        // Append only new entry
        for entry in self.unflushed_history() {
            writeln!(writer, "{entry}")?;
        }

        writer.flush()?;

        // Set new flush index
        self.mark_flushed();

        Ok(())
    }
}
