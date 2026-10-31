// write_blob.rs — stream file bytes into the store, returning their id and length.
// Why: hashing and compressing in one pass means a file is never held whole in memory.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::store::read::object_exists;
use crate::store::write::{finalize_object, stream_blob, temp_object};
use crate::Result;
use std::io::Read;
use std::path::Path;

pub fn write_blob<R: Read>(root: &Path, reader: R) -> Result<(Hash, u64)> {
    let mut temporary = temp_object(root)?; // written in .grit/tmp, then renamed into place
    let (hash, size) = stream_blob(reader, temporary.as_file_mut())?; // hash and compress in one pass
    if object_exists(root, &hash) {
        return Ok((hash, size)); // identical content already stored; the temp file is dropped
    }
    finalize_object(root, &hash, temporary)?;
    Ok((hash, size))
}
