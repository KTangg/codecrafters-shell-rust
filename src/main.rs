use std::env;
use std::io::{self, Write};

mod commands;
mod context;
mod job;
mod lexer;

use commands::builtins;
use job::Job;
use lexer::Lexer;

use crate::context::{Env, ShellContext};

fn main() {
    let mut ctx = ShellContext {
        registry: &builtins::init_registry(),
        env: &Env::from_system(),
        cwd: env::current_dir().unwrap(),
    };

    loop {
        prompt();
        let mut lex = Lexer::new();
        let input = read_user_input();
        lex.push(&input.trim());

        let tokens = lex.tokenize();

        if tokens.is_empty() {
            continue;
        }

        let job = Job::new(tokens);

        job.run(&mut ctx);
    }
}

fn prompt() {
    // TODO CWD mode & > mode promp
    print!("$ ");

    io::stdout().flush().expect("failed to flush stdout");
}

fn read_user_input() -> String {
    let mut buf = String::new();

    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read stdin");

    buf
}
