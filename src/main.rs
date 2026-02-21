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

use crate::context::ShellContext;
use crate::readline::make_readline_helper;

fn main() {
    let mut ctx = ShellContext::new();

    let mut lex = Lexer::new();

    let config = Config::builder()
        .auto_add_history(true)
        .completion_type(CompletionType::List)
        .build();

    let mut editor = Editor::with_config(config).expect("failed to initiate editor");

    // editor.set_auto_add_history(true);
    // editor.set_completion_show_all_if_ambiguous(true);

    loop {
        editor.set_helper(Some(make_readline_helper(&ctx)));

        let readline = editor.readline("$ ");
        match readline {
            Ok(line) => {
                lex.push(&line.trim());

                let tokens = lex.tokenize();
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
