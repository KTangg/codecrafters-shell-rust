use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

use crate::context::ShellContext;

pub struct ExtCommand;

impl ExtCommand {
    pub fn search_binary(name: &str, ctx: &ShellContext) -> Option<PathBuf> {
        let Some(paths) = ctx.env.get("PATH") else {
            todo!();
        };

        for path in std::env::split_paths(paths) {
            let bin_path = path.join(name);

            if let Ok(meta) = std::fs::metadata(&bin_path) {
                if meta.is_file() && (meta.permissions().mode() & 0o001 != 0) {
                    return Some(bin_path);
                }
            }
        }

        eprintln!("{name}: not found");
        None
    }

    pub fn execute(name: &str, args: &[String], ctx: &ShellContext) {
        let Some(path) = ExtCommand::search_binary(name, ctx) else {
            return;
        };

        let mut child = Command::new(path)
            .arg0(name)
            .args(args)
            .spawn()
            .expect("{name} command failed to start");

        child.wait().expect("{name} command wasn't running");
    }
}
