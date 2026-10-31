use super::Lock;
use crate::store::paths::lock_path;
use crate::{GritError, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

impl Lock {
    pub fn acquire(root: &Path) -> Result<Self> {
        let path = lock_path(root);
        if let Ok(owner) = fs::read_to_string(&path) {
            if owner.trim().parse::<u32>().is_ok_and(process_is_live) {
                return Err(GritError::Locked(owner.trim().parse().unwrap()));
            }
            fs::remove_file(&path)?;
        }
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(lock_error)?;
        write!(file, "{}", std::process::id())?;
        file.sync_all()?;
        Ok(Self {
            path,
            acquired: true,
        })
    }
}

fn process_is_live(pid: u32) -> bool {
    unsafe { libc::kill(pid as i32, 0) == 0 }
}

fn lock_error(error: std::io::Error) -> GritError {
    if error.kind() == std::io::ErrorKind::AlreadyExists {
        GritError::TransactionConflict("repository lock raced".into())
    } else {
        GritError::Io(error)
    }
}
