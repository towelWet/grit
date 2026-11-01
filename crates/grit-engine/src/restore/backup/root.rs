// root.rs — one item split out of backup.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn root(repository: &Path) -> PathBuf {
    crate::store::paths::tmp_dir(repository)
        .join("restore-backups")
        .join(Utc::now().format("%Y%m%d%H%M%S%f").to_string())
}
