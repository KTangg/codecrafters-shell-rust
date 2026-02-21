mod commands;
mod context;
mod job;
mod lexer;
mod readline;

use job::Job;
use lexer::Lexer;

use readline::ReadlineError;
use rustyline::Editor;
use rustyline::config::Configurer;

use crate::context::ShellContext;
use crate::readline::make_readline_helper;

fn main() {
    let mut ctx = ShellContext::new();

    let mut lex = Lexer::new();

    // let builtin_names = ctx
    //     .builtin_names_iter()
    //     .map(|name| name.to_string())
    //     .collect();
    let mut editor = Editor::new().expect("failed to initiate editor");
    editor.set_auto_add_history(true);

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
