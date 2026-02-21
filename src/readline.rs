pub use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{Editor, completion::Completer};

pub fn initiate_editor(
    builtin_names: Vec<String>,
) -> Result<Editor<ReadlineHelper, FileHistory>, ReadlineError> {
    let mut editor = Editor::new()?;
    editor.set_helper(Some(ReadlineHelper { builtin_names }));

    Ok(editor)
}

pub struct ReadlineHelper {
    builtin_names: Vec<String>,
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

        if prefix.len() < 3 {
            return Ok((start, Vec::new()));
        }

        let matches: Vec<String> = self
            .builtin_names
            .iter()
            .filter(|name| name.starts_with(prefix))
            .map(|s| s.to_string())
            .collect();

        Ok((start, matches))
    }
}

impl rustyline::hint::Hinter for ReadlineHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        let (start, candidates) = self.complete(line, pos, ctx).ok()?;
        let candidate = candidates.get(0)?;

        let typed_len = pos.checked_sub(start)?;

        candidate.get(typed_len..).map(|s| s.to_string())
    }
}

impl rustyline::highlight::Highlighter for ReadlineHelper {}
impl rustyline::validate::Validator for ReadlineHelper {}
