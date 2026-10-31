use crate::store::paths::tags_dir;
use crate::{GritError, Hash, Result};
use std::fs;
use std::path::Path;

pub fn set_tag(root: &Path, tag: &str, hash: &Hash) -> Result<()> {
    super::name::validate(tag)?;
    let path = tags_dir(root).join(tag);
    if path.exists() {
        return Err(GritError::TagExists(tag.into()));
    }
    crate::store::write::atomic_write(&path, hash.hex().as_bytes())
}

pub fn get_tag(root: &Path, tag: &str) -> Result<Hash> {
    super::name::validate(tag)?;
    let value = fs::read_to_string(tags_dir(root).join(tag))
        .map_err(|_| GritError::RefNotFound(tag.into()))?;
    Hash::from_hex(value.trim())
}

pub fn list_tags(root: &Path) -> Result<Vec<String>> {
    super::branch::list_names(&tags_dir(root))
}
