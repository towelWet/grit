// new.rs — build an in-memory Blob from bytes already read.
// Why: the legacy shape records its own length; decoded raw blobs reuse the same struct.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::Blob;

impl Blob {
    pub fn new(data: Vec<u8>) -> Self {
        let size = data.len() as u64; // computed once so callers never disagree with data
        Self { data, size }
    }
}
