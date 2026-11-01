// impl_drop.rs — one item split out of lock.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl Drop for Lock {
    fn drop(&mut self) {
        self.release();
    }
}
