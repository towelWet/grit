use crate::store::paths::grit_dir;
use crate::{GritError, Result};
use std::path::{Path, PathBuf};

pub fn root(path: &Path) -> Result<PathBuf> {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let mut current = Some(canonical.as_path());
    while let Some(directory) = current {
        if grit_dir(directory).exists() {
            return Ok(directory.to_path_buf());
        }
        current = directory.parent();
    }
    Err(GritError::NotARepository(path.display().to_string()))
}
