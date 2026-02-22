mod commands;
mod context;
mod job;
mod lexer;
mod readline;

use job::Job;
use lexer::Lexer;

use readline::ReadlineError;
use rustyline::config::Config;
use rustyline::history::History;
use rustyline::{CompletionType, Editor, Helper};

use context::ShellContext;
use readline::make_readline_helper;

fn main() {
    let mut ctx = ShellContext::new();

    let mut lex = Lexer::new();

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut editor = Editor::with_config(config).expect("failed to initiate editor");

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
}

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
