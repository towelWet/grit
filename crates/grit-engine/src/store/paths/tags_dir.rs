// tags_dir.rs — one item split out of paths.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn tags_dir(r: &Path) -> PathBuf {
    refs_dir(r).join("tags")
}
