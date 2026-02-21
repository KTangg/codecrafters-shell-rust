use crate::commands::ExtCommand;
use crate::context::ShellContext;
use crate::lexer::Token;

use libc::{close, dup, dup2, fork, waitpid};
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

struct FdGuard {
    fd: i32,
    saved: i32,
}

impl FdGuard {
    fn new(fd: i32) -> Self {
        let saved = unsafe { dup(fd) };
        Self { fd, saved }
    }
}

impl Drop for FdGuard {
    fn drop(&mut self) {
        unsafe {
            dup2(self.saved, self.fd);
            close(self.saved);
        }
    }
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
        match self.commands.as_slice() {
            [unit] => {
                // exactly one command
                let name = unit.get_name().unwrap();

                let is_builtin = { ctx.registry().get_command(name).is_some() };

                if is_builtin {
                    // Just Some fd restoration
                    let _stdout_guard = FdGuard::new(1);
                    let _stderr_guard = FdGuard::new(2);

                    Self::redirect_output(unit.get_outputs());
                    Self::exec(unit, ctx);
                } else {
                    Self::execute_unit(unit, ctx);
                }
            }
            units => {
                for unit in units {
                    Self::execute_unit(unit, ctx);
                }
            }
        }
    }

    fn execute_unit(unit: &CommandUnit, ctx: &mut ShellContext) {
        match unsafe { fork() } {
            0 => {
                // CHILD Process
                Self::redirect_output(unit.get_outputs());
                Self::exec(unit, ctx);
                std::process::exit(1);
            }
            pid => {
                // Parent Process
                unsafe { waitpid(pid, std::ptr::null_mut(), 0) };
            }
        }
    }

    fn exec(unit: &CommandUnit, ctx: &mut ShellContext) {
        let (name, args) = unit.argv.split_first().unwrap();

        if let Some(builtin) = ctx.registry().get_command(name) {
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

    fn get_name(&self) -> Option<&String> {
        self.argv.get(0)
    }

    fn get_outputs(&self) -> &Vec<Output> {
        &self.outputs
    }
}
