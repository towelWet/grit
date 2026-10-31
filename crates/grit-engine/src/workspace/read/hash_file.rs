// hash_file.rs — the content id of a file read from disk in fixed-size chunks (cache-miss path only).
// Why: hashing a large file must not load it whole; the stat cache exists to avoid calling this at all.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::{blob_hasher, Hash};
use crate::Result;
use std::fs::File;
use std::path::Path;

pub(super) fn hash_file(path: &Path) -> Result<Hash> {
    let mut hasher = blob_hasher(); // domain tag first, then the file content
    hasher.update_reader(File::open(path)?)?; // blake3 reads through a fixed-size buffer
    Ok(Hash(*hasher.finalize().as_bytes()))
}
