// objects_dir.rs — one item split out of mod.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

impl Repository {
    pub fn objects_dir(&self) -> PathBuf {
        crate::store::paths::objects_dir(&self.root)
    }
}
