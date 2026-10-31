// object.rs — check one object file: it must decompress and hash to the name it is stored under.
// Why: content addressing is the integrity guarantee; a renamed or bit-flipped file must be caught here.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::{hash_to_storage_path, Hash};
use crate::objects::payload_hash::payload_hash;
use crate::{GritError, Result};
use std::fs;
use std::path::Path;

pub fn verify(objects: &Path, path: &Path) -> Result<()> {
    let compressed = fs::read(path)?;
    let raw =
        zstd::decode_all(&compressed[..]).map_err(|error| GritError::Zstd(error.to_string()))?;
    let hash: Hash = payload_hash(&raw); // raw blobs and JSON objects use different id rules
    if path != objects.join(hash_to_storage_path(&hash)) {
        return Err(GritError::ObjectCorrupt(
            path.display().to_string(),
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            hash.hex(),
        ));
    }
    Ok(())
}
