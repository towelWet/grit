use crate::{GritError, Hash, Result};
use std::path::Path;

pub fn resolve_ref(root: &Path, name: &str) -> Result<Hash> {
    match name {
        "HEAD" => super::get_head_hash(root),
        value if value.starts_with("refs/heads/") => super::get_branch_ref(root, &value[11..]),
        value if value.starts_with("refs/tags/") => super::get_tag(root, &value[10..]),
        _ => super::get_branch_ref(root, name).or_else(|_| super::get_tag(root, name)),
    }
}

pub fn resolve_revision(root: &Path, value: &str) -> Result<Hash> {
    if let Ok(hash) = resolve_ref(root, value) {
        return Ok(hash);
    }
    if let Some(result) = super::ancestor::resolve_ancestor(root, value) {
        return result;                                       // `<rev>~N` form (refs/ancestor.rs)
    }
    if value.len() == 64 {
        let hash = Hash::from_hex(value)?;
        crate::store::read::read_commit(root, &hash)?;
        return Ok(hash);
    }
    if value.len() < 6 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(GritError::SaveNotFound(value.into()));
    }
    let matches = super::prefix::commit_matches(root, value)?;
    match matches.as_slice() {
        [hash] => Ok(*hash),
        [] => Err(GritError::SaveNotFound(value.into())),
        _ => Err(GritError::InvalidObject(format!(
            "ambiguous revision: {value}"
        ))),
    }
}
