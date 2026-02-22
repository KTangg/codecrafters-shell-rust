use std::{collections::HashMap, io, path::PathBuf};

use libc::{close, dup, dup2};

pub enum Redirect {
    Write { fd: i32, target: PathBuf },
    Append { fd: i32, target: PathBuf },
}

pub struct FdGuard {
    saved: HashMap<i32, i32>,
}

impl FdGuard {
    pub fn new() -> Self {
        Self {
            saved: HashMap::new(),
        }
    }

    pub fn save(&mut self, fd: i32) -> io::Result<()> {
        // Already save key/value no overwrited
        if self.saved.contains_key(&fd) {
            return Ok(());
        }

        // Create fd backup
        let backup = unsafe { dup(fd) };
        if backup < 0 {
            return Err(io::Error::last_os_error());
        }

        self.saved.insert(fd, backup);
        Ok(())
    }

    pub fn redirect(&mut self, src: i32, dst: i32) -> io::Result<()> {
        self.save(dst)?;

        if unsafe { dup2(src, dst) } < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for FdGuard {
    fn drop(&mut self) {
        for (fd, backup) in self.saved.drain() {
            // Redirect back and close the backup
            unsafe {
                // Ignore errors during restoration (best-effort cleanup)
                dup2(backup, fd);
                close(backup);
            }
        }
    }
}
