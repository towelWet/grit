// blob_hasher.rs — BLAKE3 state pre-seeded with the blob domain tag.
// Why: a raw blob's id must never equal a tree or commit JSON id, so content is hashed after "blob\0".
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
const BLOB_DOMAIN: &[u8] = b"blob\0"; // NUL cannot start JSON, so no tree or commit can collide

pub fn blob_hasher() -> blake3::Hasher {
    let mut hasher = blake3::Hasher::new();
    hasher.update(BLOB_DOMAIN); // callers feed content after the tag, in chunks if they stream
    hasher
}
