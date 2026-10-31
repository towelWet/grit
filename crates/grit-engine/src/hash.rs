// hash.rs — the 32-byte BLAKE3 identity of every stored object.
// Why: one type carries ids across blobs, trees, commits, and refs; each method lives in its own file.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
mod as_bytes;
mod blob_hasher;
mod display;
mod from_hex;
mod of;
mod of_blob;
mod serde_impl;
mod short;
mod storage_path;
mod to_hex;

pub use blob_hasher::blob_hasher;
pub use storage_path::hash_to_storage_path;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash(pub [u8; 32]); // raw digest bytes; Display/serde render them as hex
