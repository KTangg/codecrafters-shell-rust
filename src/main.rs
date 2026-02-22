mod commands;
mod context;
mod job;
mod lexer;
mod readline;

use std::io;
use std::path::{Path, PathBuf};

use job::Job;
use lexer::Lexer;

use readline::ReadlineError;
use rustyline::config::Config;
use rustyline::history::History;
use rustyline::{CompletionType, Editor, Helper};

use context::ShellContext;
use readline::make_readline_helper;

fn main() -> io::Result<()> {
    let mut ctx = ShellContext::new();

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();
    let mut editor = Editor::with_config(config).expect("failed to initiate editor");

    // Preload history
    let history_file_path = PathBuf::from(ctx.env("HISTFILE").unwrap_or(""));
    let _ = preload_history(&mut ctx, &mut editor, &history_file_path);

    let mut lex = Lexer::new();
    loop {
        editor.set_helper(Some(make_readline_helper(&ctx))); // Not efficient since we need to copied some ctx value everytime

        let readline = editor.readline("$ ");
        match readline {
            Ok(line) => {
                // Preprocess line
                let line = line.trim();

                // Add line to history which consume line this should move up
                add_history(&mut ctx, &mut editor, line);

                // Parsing line
                lex.push(&line);
                let tokens = lex.tokenize();

                // Run job
                let job = Job::new(tokens);
                job.run(&mut ctx);
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("{e:?}");
                break;
            }
        }
    }

    ctx.flush_history(&PathBuf::from(ctx.env("HISTFILE").unwrap_or("")))?;
    Ok(())
}

// TODO move these utils out

fn add_history<H, I>(ctx: &mut ShellContext, editor: &mut Editor<H, I>, line: &str)
where
    H: Helper,
    I: History,
{
    // Keep history in sync on both rustyline and builtin
    if let Ok(result) = editor.add_history_entry(line) {
        if result {
            ctx.push_history(line);
        }
    }
}

fn preload_history<H, I>(
    ctx: &mut ShellContext,
    editor: &mut Editor<H, I>,
    path: &Path,
) -> io::Result<()>
where
    H: Helper,
    I: History,
{
    let _ = editor.load_history(path);
    ctx.preload_history(path)?;

    Ok(())
}
