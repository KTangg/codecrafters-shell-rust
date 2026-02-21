mod commands;
mod context;
mod job;
mod lexer;
mod readline;

use job::Job;
use lexer::Lexer;
use readline::ReadlineError;
use rustyline::config::Configurer;

use crate::context::ShellContext;

fn main() {
    let mut ctx = ShellContext::new();

    let mut lex = Lexer::new();

    let builtin_names = ctx
        .builtin_names_iter()
        .map(|name| name.to_string())
        .collect();
    let mut editor = readline::initiate_editor(builtin_names).expect("failed to initiate readline");
    editor.set_auto_add_history(true);

    loop {
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
