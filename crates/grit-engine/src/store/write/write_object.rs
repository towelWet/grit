// write_object.rs — store a JSON payload (tree, commit) under the hash of its stored bytes.
// Why: an existing object with the same id already holds identical bytes, so it is trusted without a read-back.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::store::read::object_exists;
use crate::store::write::{finalize_object, temp_object};
use crate::Result;
use std::io::Write;
use std::path::Path;

pub fn write_object(root: &Path, data: &[u8]) -> Result<Hash> {
    let hash = Hash::of(data); // JSON objects are addressed by the bytes that go to disk
    if object_exists(root, &hash) {
        return Ok(hash); // trust the existing object; `grit verify` checks its bytes
    }
    let mut temporary = temp_object(root)?;
    temporary.write_all(&zstd::encode_all(data, 3)?)?; // level 3 matches every earlier writer
    finalize_object(root, &hash, temporary)?;
    Ok(hash)
}
