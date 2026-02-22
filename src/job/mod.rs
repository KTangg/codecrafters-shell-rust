use super::ShellContext;
use super::lexer::Token;

use libc::{dup2, fork, waitpid};
use std::path::PathBuf;
use std::{io, os::fd::AsRawFd};

mod process;
mod redirect;

use process::Process;
use redirect::{FdGuard, Redirect};

pub struct Job {
    processes: Vec<Process>,
}

impl Job {
    pub fn new(tokens: Vec<Token>) -> Job {
        let mut processes = Vec::<Process>::new();

        let mut current_process = Process::new();
        let mut iter = tokens.into_iter().peekable();
        while let Some(tk) = iter.next() {
            match tk {
                Token::Literal(arg) => {
                    current_process.push_arg(arg);
                }
                // Token::Complex(arg) => {
                //     todo!()
                // }
                Token::Write(fd) => {
                    let Some(Token::Literal(target)) = iter.peek() else {
                        todo!()
                    };
                    current_process.push_output(Redirect::Write {
                        fd,
                        target: PathBuf::from(target),
                    });
                    iter.next();
                }
                Token::Append(fd) => {
                    let Some(Token::Literal(target)) = iter.peek() else {
                        todo!()
                    };
                    current_process.push_output(Redirect::Append {
                        fd,
                        target: PathBuf::from(target),
                    });
                    iter.next();
                }
                Token::Pipe => {
                    processes.push(current_process);
                    current_process = Process::new();
                }
            }
        }

        // TODO a condition check
        processes.push(current_process);

        Job { processes }
    }

    pub fn processes(&self) -> impl Iterator<Item = &Process> {
        self.processes.iter()
    }

    fn is_single_builtin(&self, ctx: &ShellContext) -> bool {
        self.processes.len() == 1 && self.processes[0].is_builtin(ctx)
    }

    pub fn run(&self, ctx: &mut ShellContext) {
        if self.is_single_builtin(ctx) {
            // Run single builtin without forking the process
            self.processes[0].execute(ctx);
            return;
        }

        let mut pids = Vec::new();
        let mut prev_read: Option<io::PipeReader> = None;

        // Run multiple process pipeline || single process external binary
        for (i, process) in self.processes().enumerate() {
            let is_last = i == self.processes.len() - 1;

            // Create pipe
            let mut next_pipe = None;
            if !is_last {
                next_pipe = Some(io::pipe().expect("pipe failed"));
            }

            match unsafe { fork() } {
                0 => {
                    // Read from prev pipe
                    if let Some(ref prev) = prev_read {
                        unsafe {
                            dup2(prev.as_raw_fd(), 0);
                        }
                    }

                    // Write to next pipe
                    if let Some((_, ref write_pipe)) = next_pipe {
                        unsafe { dup2(write_pipe.as_raw_fd(), 1) };
                    }

                    // Close unused end / prev pipe read && self pipe that alreadys dup2
                    drop(prev_read);
                    drop(next_pipe);

                    process.execute(ctx);
                    std::process::exit(1);
                }

                pid => {
                    // Parent collect pid
                    pids.push(pid);

                    // Parent doesn't need to read from prev read end
                    drop(prev_read);

                    // Parent keeps next read end
                    prev_read = next_pipe.map(|(r, _)| r);
                }
            }
        }

        // Wait for all pid
        for pid in pids {
            unsafe { waitpid(pid, std::ptr::null_mut(), 0) };
        }
    }
}
