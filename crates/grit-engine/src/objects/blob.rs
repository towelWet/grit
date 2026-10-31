// blob.rs — file contents: raw bytes behind a 4-byte marker, or the legacy JSON shape.
// Why: JSON byte arrays cost ~3.5x the file size; old repos still hold JSON, so both must decode.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use serde::{Deserialize, Serialize};

mod new;
mod raw_payload;
pub use raw_payload::raw_payload;

// First four bytes of a raw blob payload. JSON payloads start with '{', so the marker is unambiguous.
pub const RAW_MAGIC: &[u8; 4] = b"GRB1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    pub data: Vec<u8>, // decoded file bytes, whichever format they were stored in
    pub size: u64,     // length of data, kept for the legacy JSON shape
}
