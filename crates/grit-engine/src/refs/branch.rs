use crate::store::paths::heads_dir;
use crate::{GritError, Hash, Result};
use std::fs;
use std::path::Path;

pub fn set_branch_ref(root: &Path, branch: &str, hash: &Hash) -> Result<()> {
    super::name::validate(branch)?;
    crate::store::write::atomic_write(&heads_dir(root).join(branch), hash.hex().as_bytes())
}

pub fn get_branch_ref(root: &Path, branch: &str) -> Result<Hash> {
    super::name::validate(branch)?;
    let value = fs::read_to_string(heads_dir(root).join(branch))
        .map_err(|_| GritError::RefNotFound(branch.into()))?;
    Hash::from_hex(value.trim())
}

pub fn list_branches(root: &Path) -> Result<Vec<String>> {
    list_names(&heads_dir(root))
}

pub(super) fn list_names(directory: &Path) -> Result<Vec<String>> {
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut names: Vec<_> = fs::read_dir(directory)?
        .filter_map(|entry| {
            entry
                .ok()
                .map(|e| e.file_name().to_string_lossy().to_string())
        })
        .collect();
    names.sort();
    Ok(names)
}
