// raw_payload.rs — return the file bytes when a payload is a raw blob, else None.
// Why: readers and verify must tell raw blobs from JSON objects without parsing either one.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::RAW_MAGIC;

pub fn raw_payload(payload: &[u8]) -> Option<&[u8]> {
    payload.strip_prefix(RAW_MAGIC.as_slice()) // Some(bytes after the marker) only for raw blobs
}
