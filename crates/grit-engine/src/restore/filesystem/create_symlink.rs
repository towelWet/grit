// create_symlink.rs — one item split out of filesystem.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(unix)]
pub fn create_symlink(path: &Path, target: &[u8]) -> Result<()> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::symlink;
    let temporary = path.with_extension(format!("grit-restore-{}", std::process::id()));
    symlink(OsStr::from_bytes(target), &temporary)?;
    fs::rename(temporary, path)?;
    Ok(())
}
