// create_symlink_.rs — one item split out of filesystem.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(not(unix))]
pub fn create_symlink(_: &Path, _: &[u8]) -> Result<()> {
    Err(GritError::Internal("symlink restore is unsupported".into()))
}
