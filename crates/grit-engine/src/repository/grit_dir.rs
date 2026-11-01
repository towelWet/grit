// grit_dir.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl Repository {
    pub fn grit_dir(&self) -> PathBuf {
        crate::store::paths::grit_dir(&self.root)
    }
}
