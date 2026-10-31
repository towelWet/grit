// read_blob.rs — return a file's bytes whether it was stored raw or as legacy JSON.
// Why: old repos keep JSON blobs forever (no forced migration), so every reader must accept both.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::blob::raw_payload;
use crate::objects::Blob;
use crate::store::read::read_object;
use crate::{GritError, Result};
use std::path::Path;

pub fn read_blob(root: &Path, hash: &Hash) -> Result<Blob> {
    let data = read_object(root, hash)?;
    match raw_payload(&data) {
        Some(raw) => Ok(Blob::new(raw.to_vec())), // raw: the bytes after the marker are the file
        None => serde_json::from_slice(&data).map_err(GritError::Json), // legacy: JSON {data, size}
    }
}
