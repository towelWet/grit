// set_head.rs — one item split out of head.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn set_head(root: &Path, reference: &str) -> Result<()> {
    crate::store::write::atomic_write(&head_path(root), format!("{reference}\n").as_bytes())
}
