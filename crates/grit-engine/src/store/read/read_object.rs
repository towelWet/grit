// read_object.rs — load one object file and return its decompressed payload.
// Why: every object is zstd-framed on disk; callers decode the payload according to its kind.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::{hash_to_storage_path, Hash};
use crate::store::paths::objects_dir;
use crate::{GritError, Result};
use std::fs;
use std::path::Path;

pub fn read_object(root: &Path, hash: &Hash) -> Result<Vec<u8>> {
    let path = objects_dir(root).join(hash_to_storage_path(hash));
    let compressed = fs::read(&path).map_err(|_| GritError::ObjectNotFound(hash.hex()))?; // missing file => not found
    let data = zstd::decode_all(&compressed[..]).map_err(|e| GritError::Zstd(e.to_string()))?;
    Ok(data)
}
