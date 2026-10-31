use crate::Result;
use std::fs;
use std::path::Path;

#[cfg(unix)]
pub fn file_mode(metadata: &fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    metadata.mode()
}

#[cfg(not(unix))]
pub fn file_mode(metadata: &fs::Metadata) -> u32 {
    if metadata.is_dir() {
        0o040755
    } else {
        0o100644
    }
}

#[cfg(unix)]
pub fn symlink_target(path: &Path) -> Result<Vec<u8>> {
    use std::os::unix::ffi::OsStrExt;
    Ok(fs::read_link(path)?.as_os_str().as_bytes().to_vec())
}

#[cfg(not(unix))]
pub fn symlink_target(path: &Path) -> Result<Vec<u8>> {
    Ok(fs::read_link(path)?.to_string_lossy().as_bytes().to_vec())
}
