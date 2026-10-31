// of_blob.rs — content id of a raw file: domain-tagged BLAKE3 of its bytes.
// Why: files are stored as raw bytes, so their id must come from those bytes, not from a JSON wrapper.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::blob_hasher::blob_hasher;
use super::Hash;

impl Hash {
    pub fn of_blob(data: &[u8]) -> Self {
        let mut hasher = blob_hasher(); // starts with the "blob\0" tag
        hasher.update(data);
        Self(*hasher.finalize().as_bytes())
    }
}
