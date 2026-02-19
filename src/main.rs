use std::env;
use std::io::{self, Write};

mod commands;
mod context;
mod lexer;

use commands::builtins;
use lexer::{Lexer, Token};

use crate::commands::ExtCommand;
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

        // println!("{tokens:?}");

        let args: Vec<String> = tokens
            .into_iter()
            .filter_map(|t| {
                if let Token::Literal(s) = t {
                    Some(s)
                } else {
                    None
                }
            })
            .collect();

        let (cmd_name, cmd_args) = args.split_first().unwrap();

        let Some(cmd) = ctx.registry.get_command(cmd_name) else {
            ExtCommand::execute(cmd_name, cmd_args, &ctx);
            continue;
        };

        cmd.execute(&cmd_args, &mut ctx);
    }
}

fn prompt() {
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
