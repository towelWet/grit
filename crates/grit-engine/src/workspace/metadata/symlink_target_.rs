// symlink_target_.rs — one item split out of metadata.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(not(unix))]
pub fn symlink_target(path: &Path) -> Result<Vec<u8>> {
    Ok(fs::read_link(path)?.to_string_lossy().as_bytes().to_vec())
}
