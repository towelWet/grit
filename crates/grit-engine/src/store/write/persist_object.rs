// persist_object — move a finished object temp file into its content-addressed path.
// Why: persist_noclobber uses renamex_np(RENAME_EXCL) on macOS, which exFAT/FAT
// volumes reject with ENOTSUP (os error 45), so every snapshot there failed.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::is_unsupported::is_unsupported;
use crate::{GritError, Result};
use std::path::Path;
use tempfile::NamedTempFile;

pub fn persist_object(tmp: NamedTempFile, path: &Path) -> Result<()> {
    let error = match tmp.persist_noclobber(path) {           // preferred: atomic, refuses to overwrite
        Ok(_) => return Ok(()),
        Err(error) => error,
    };
    if error.error.kind() == std::io::ErrorKind::AlreadyExists {
        return Ok(());                                        // same hash already stored by a racer
    }
    if !is_unsupported(&error.error) {
        return Err(GritError::Internal(format!("write: {error}")));
    }
    if path.exists() {
        return Ok(());                                        // content-addressed: existing bytes are equal
    }
    error
        .file
        .persist(path)                                        // plain rename works on exFAT; a lost race
        .map_err(|e| GritError::Internal(format!("write: {e}")))?; // can only replace identical bytes
    Ok(())                                                    // no read-back: an existing object is trusted; `grit verify` checks its bytes
}
