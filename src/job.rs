use crate::commands::ExtCommand;
use crate::context::ShellContext;
use crate::lexer::Token;

use libc::{dup2, fork, waitpid};
use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::PathBuf;

pub struct Job {
    commands: Vec<CommandUnit>,
}

enum Output {
    Write { fd: usize, target: PathBuf },
    Append { fd: usize, target: PathBuf },
}

struct CommandUnit {
    argv: Vec<String>,
    outputs: Vec<Output>,
}

impl Job {
    pub fn new(tokens: Vec<Token>) -> Job {
        let mut commands = Vec::<CommandUnit>::new();
        let mut command_unit = CommandUnit::new();

        let mut iter = tokens.into_iter().peekable();
        while let Some(tk) = iter.next() {
            match tk {
                Token::Literal(arg) => {
                    command_unit.push_arg(arg);
                }
                Token::Complex(arg) => {
                    todo!()
                }
                Token::Redirect(fd) => {
                    let Some(Token::Literal(target)) = iter.peek() else {
                        todo!()
                    };
                    command_unit.push_output(Output::Write {
                        fd,
                        target: PathBuf::from(target),
                    });
                    iter.next();
                }
                Token::Append(fd) => {
                    let Some(Token::Literal(target)) = iter.peek() else {
                        todo!()
                    };
                    command_unit.push_output(Output::Append {
                        fd,
                        target: PathBuf::from(target),
                    });
                    iter.next();
                }
                Token::Pipe => {
                    commands.push(command_unit);
                    command_unit = CommandUnit::new();
                }
            }
        }

        // TODO a condition check
        commands.push(command_unit);

        Job { commands }
    }

    pub fn run(self, ctx: &mut ShellContext) {
        // TODO implement piping
        for unit in self.commands {
            Self::execute_unit(unit, ctx);
        }
    }

    fn execute_unit(unit: CommandUnit, ctx: &mut ShellContext) {
        match unsafe { fork() } {
            0 => {
                // CHILD Process
                Self::redirect_output(&unit.outputs);
                Self::exec(unit, ctx);
                std::process::exit(1);
            }
            pid => {
                // Parent Process
                unsafe { waitpid(pid, std::ptr::null_mut(), 0) };
            }
        }
    }

    fn exec(unit: CommandUnit, ctx: &mut ShellContext) {
        let (name, args) = unit.argv.split_first().unwrap();

        if let Some(builtin) = ctx.registry.get_command(name) {
            builtin.execute(args, ctx);
        } else {
            ExtCommand::execute(name, args, ctx);
        }
    }

    fn redirect_output(outputs: &[Output]) {
        for output in outputs {
            match output {
                Output::Write { fd, target } => {
                    let file = File::options()
                        .create(true)
                        .write(true)
                        .open(target)
                        .unwrap();

                    let file_fd = file.as_raw_fd();
                    unsafe {
                        dup2(file_fd, *fd as i32);
                    }
                }
                Output::Append { fd, target } => {
                    let file = File::options()
                        .create(true)
                        .append(true)
                        .open(target)
                        .unwrap();
                    let file_fd = file.as_raw_fd();

                    unsafe {
                        dup2(file_fd, *fd as i32);
                    }
                }
            }
        }
    }
}

impl CommandUnit {
    fn new() -> CommandUnit {
        CommandUnit {
            argv: Vec::<String>::new(),
            outputs: Vec::<Output>::new(),
        }
    }

    fn push_arg(&mut self, arg: String) {
        self.argv.push(arg);
    }

    fn push_output(&mut self, output: Output) {
        self.outputs.push(output);
    }
}
