use crate::store::paths::head_path;
use crate::{GritError, Hash, Result};
use std::fs;
use std::path::Path;

pub fn get_head(root: &Path) -> Result<String> {
    let path = head_path(root);
    if !path.exists() {
        return Err(GritError::RefNotFound("HEAD".into()));
    }
    Ok(fs::read_to_string(path)?.trim().to_string())
}

pub fn set_head(root: &Path, reference: &str) -> Result<()> {
    crate::store::write::atomic_write(&head_path(root), format!("{reference}\n").as_bytes())
}

pub fn get_current_branch(root: &Path) -> Result<String> {
    get_head(root)?
        .strip_prefix("ref: refs/heads/")
        .map(|branch| branch.trim().to_string())
        .ok_or_else(|| GritError::Internal("detached".into()))
}

pub fn get_head_hash(root: &Path) -> Result<Hash> {
    let head = get_head(root)?;
    if let Some(branch) = head.strip_prefix("ref: refs/heads/") {
        super::get_branch_ref(root, branch)
    } else {
        Hash::from_hex(&head)
    }
}
