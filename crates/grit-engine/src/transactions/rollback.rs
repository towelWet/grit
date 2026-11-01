// rollback.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn rollback(root: &Path, id: &str) -> Result<()> {
    upd(root, id, "rolled_back")
}
