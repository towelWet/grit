// impl_debug.rs — one item split out of display.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl fmt::Debug for Hash {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "Hash({})", self.short())
    }
}
