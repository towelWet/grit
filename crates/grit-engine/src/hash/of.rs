// of.rs — BLAKE3 of the exact stored bytes; used for JSON objects (trees, commits, legacy blobs).
// Why: a JSON object's id is the hash of the bytes that sit on disk, so verify can recompute it.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Hash;

impl Hash {
    pub fn of(data: &[u8]) -> Self {
        Self(*blake3::hash(data).as_bytes()) // one-shot digest; no domain tag for JSON payloads
    }
}
