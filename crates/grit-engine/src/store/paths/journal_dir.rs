// journal_dir.rs — one item split out of paths.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn journal_dir(r: &Path) -> PathBuf {
    grit_dir(r).join("journal")
}
