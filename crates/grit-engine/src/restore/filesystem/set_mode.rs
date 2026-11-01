// set_mode.rs — one item split out of filesystem.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(unix)]
pub fn set_mode(path: &Path, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::symlink_metadata(path)?.permissions();
    permissions.set_mode(mode & 0o7777);
    fs::set_permissions(path, permissions)?;
    Ok(())
}
