// finalize_object.rs — flush a finished temp object and move it to its content-addressed path.
// Why: the name must appear only after the bytes are durable, so a crash never leaves a half-written object.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::persist_object::persist_object;
use crate::hash::{hash_to_storage_path, Hash};
use crate::store::paths::objects_dir;
use crate::Result;
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn finalize_object(root: &Path, hash: &Hash, temporary: NamedTempFile) -> Result<()> {
    let path = objects_dir(root).join(hash_to_storage_path(hash));
    fs::create_dir_all(path.parent().unwrap())?; // fan-out directory may not exist yet
    temporary.as_file().sync_all()?; // bytes reach the disk before the name does
    persist_object(temporary, &path) // noclobber: a racer's identical object is accepted
}
