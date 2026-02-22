mod commands;
mod context;
mod job;
mod lexer;
mod readline;

use job::Job;
use lexer::Lexer;

use readline::ReadlineError;
use rustyline::config::Config;
use rustyline::{CompletionType, Editor};

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
                // Process line
                lex.push(&line.trim());
                let tokens = lex.tokenize();

                // Add line to history which consume line this should move up
                ctx.push_history(line);

                if tokens.is_empty() {
                    continue;
                }

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
