use std::fs;
use std::path::PathBuf;
mod acquire;

pub struct Lock {
    path: PathBuf,
    acquired: bool,
}

impl Lock {
    pub fn release(&mut self) {
        if self.acquired {
            fs::remove_file(&self.path).ok();
            self.acquired = false;
        }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        self.release();
    }
}
