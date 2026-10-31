use crate::{GritError, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn atomic_file(path: &Path, data: &[u8]) -> Result<()> {
    let mut temporary = tempfile::NamedTempFile::new_in(path.parent().unwrap())?;
    temporary.write_all(data)?;
    temporary.as_file().sync_all()?;
    temporary
        .persist(path)
        .map_err(|error| GritError::Internal(format!("restore file: {error}")))?;
    Ok(())
}

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

#[cfg(not(unix))]
pub fn create_symlink(_: &Path, _: &[u8]) -> Result<()> {
    Err(GritError::Internal("symlink restore is unsupported".into()))
}

#[cfg(unix)]
pub fn set_mode(path: &Path, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::symlink_metadata(path)?.permissions();
    permissions.set_mode(mode & 0o7777);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(unix))]
pub fn set_mode(_: &Path, _: u32) -> Result<()> {
    Ok(())
}
