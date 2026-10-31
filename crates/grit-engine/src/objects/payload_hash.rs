// payload_hash.rs — the object id implied by a stored payload.
// Why: verify and gc must recompute an id for every object, whatever format it was written in.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::blob::raw_payload;
use crate::hash::Hash;

pub fn payload_hash(payload: &[u8]) -> Hash {
    match raw_payload(payload) {
        Some(raw) => Hash::of_blob(raw), // raw blob: domain-tagged hash of the file content
        None => Hash::of(payload),       // JSON tree, commit, or legacy blob: hash of stored bytes
    }
}
