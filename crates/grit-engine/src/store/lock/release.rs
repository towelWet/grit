// release.rs — one item split out of lock.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl Lock {
    pub fn release(&mut self) {
        if self.acquired {
            fs::remove_file(&self.path).ok();
            self.acquired = false;
        }
    }
}
