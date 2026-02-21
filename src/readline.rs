use rustyline::completion::Completer;
pub use rustyline::error::ReadlineError;

use crate::context::ShellContext;
use std::{fs, path::PathBuf};

pub fn make_readline_helper(ctx: &ShellContext) -> ReadlineHelper {
    ReadlineHelper {
        builtin_names: ctx.builtin_names_iter().map(str::to_owned).collect(),
        paths: ctx.paths().collect(),
        // cwd: ctx.cwd().to_path_buf(),
    }
}

pub struct ReadlineHelper {
    builtin_names: Vec<String>,
    paths: Vec<PathBuf>,
    // cwd: PathBuf,
}

impl ReadlineHelper {
    fn complete_token(&self, prefix: &str, only_file_path: bool) -> Vec<String> {
        if only_file_path || prefix.contains('/') {
            self.search_file(prefix)
        } else {
            let mut result = Vec::new();
            result.extend(self.search_builtin(prefix));
            result.extend(self.search_path_bins(prefix));

            // dedup all duplication
            result.sort();
            result.dedup();

            // Append space if only 1 match
            if result.len() == 1 {
                result[0].push(' ');
            }

            result
        }
    }

    fn split_path(prefix: &str) -> (PathBuf, &str, bool) {
        match prefix.rfind('/') {
            Some(i) => (
                PathBuf::from(&prefix[..=i]),
                &prefix[i + 1..],
                true, // user typed a path
            ),
            None => (
                PathBuf::from("."),
                prefix,
                false, // implicit current dir
            ),
        }
    }

    fn search_file(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        let (dir, file_prefix, explicit_path) = Self::split_path(prefix);

        let Ok(entries) = fs::read_dir(&dir) else {
            return result;
        };

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            if !name.starts_with(file_prefix) {
                continue;
            }

            let mut completion = if explicit_path {
                dir.join(&*name).to_string_lossy().to_string()
            } else {
                name.to_string()
            };

            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                completion.push('/');
            }

            result.push(completion);
        }

        result.sort();
        result
    }

    fn search_builtin(&self, prefix: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for name in self.builtin_names.iter() {
            if name.starts_with(prefix) {
                // Append space behind for args input
                result.push(name.to_string());
            }
        }

        result.sort();
        result
    }

    fn search_path_bins(&self, prefix: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for dir in self.paths.iter() {
            let Ok(entries) = fs::read_dir(dir) else {
                continue;
            };

            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();

                if !name.starts_with(prefix) {
                    continue;
                }

                // TODO Check for executable

                result.push(name);
            }
        }

        result.sort();
        result
    }
}

impl rustyline::Helper for ReadlineHelper {}

impl Completer for ReadlineHelper {
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        // find start of current word
        let start = line[..pos]
            .rfind(|c: char| c.is_ascii_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);

        let prefix = line.get(start..pos).unwrap_or("");

        if prefix.len() < 2 {
            return Ok((start, Vec::new()));
        }

        // Not first argv only search for file path otherwise search all
        let only_file_path = start != 0;
        let result = self.complete_token(prefix, only_file_path);

        Ok((start, result))
    }
}

impl rustyline::hint::Hinter for ReadlineHelper {
    // Hint will fill the user input with out any action
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        // let (start, candidates) = self.complete(line, pos, ctx).ok()?;
        // let candidate = candidates.get(0)?;

        // let typed_len = pos.checked_sub(start)?;

        // candidate.get(typed_len..).map(|s| s.to_string())
        None
    }
}

impl rustyline::highlight::Highlighter for ReadlineHelper {}
impl rustyline::validate::Validator for ReadlineHelper {}
