use std::os::fd::AsRawFd;
use std::{fs::File, io};

use super::ShellContext;
use super::{FdGuard, Redirect};

use crate::commands::ExtCommand;

pub struct Process {
    argv: Vec<String>,
    redirects: Vec<Redirect>,
}

impl Process {
    pub fn new() -> Process {
        Process {
            argv: Vec::<String>::new(),
            redirects: Vec::<Redirect>::new(),
        }
    }

    pub fn push_arg(&mut self, arg: String) {
        self.argv.push(arg);
    }

    pub fn push_output(&mut self, output: Redirect) {
        self.redirects.push(output);
    }

    pub fn name(&self) -> Option<&String> {
        self.argv.get(0)
    }

    pub fn args(&self) -> &[String] {
        self.argv.get(1..).unwrap_or(&[])
    }

    pub fn is_builtin(&self, ctx: &ShellContext) -> bool {
        self.name()
            .map(|n| ctx.registry().check_builtin(n))
            .unwrap_or(false)
    }

    fn apply_redirects(&self) -> io::Result<FdGuard> {
        let mut guard = FdGuard::new();

        for r in self.redirects.iter() {
            match r {
                Redirect::Write { fd, target } => {
                    let file = File::options()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(target)?;

                    guard.redirect(file.as_raw_fd(), *fd)?
                }
                Redirect::Append { fd, target } => {
                    let file = File::options().create(true).append(true).open(target)?;

                    guard.redirect(file.as_raw_fd(), *fd)?
                }
            }
        }

        Ok(guard)
    }

    pub fn execute(&self, ctx: &mut ShellContext) {
        // Do some redirecting stuff
        let _fd_guard = match self.apply_redirects() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("failed to redirect: {e}");
                return;
            }
        };

        let (name, args) = self.argv.split_first().unwrap();

        if let Some(builtin) = ctx.registry().get_command(name) {
            builtin.execute(args, ctx);
        } else {
            ExtCommand::execute(name, args, ctx);
        }
    }
}
