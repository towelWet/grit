// temp_object.rs — create a temporary file inside .grit/tmp for an object being written.
// Why: the temp file must sit on the same volume as objects/ so the final persist is a rename, not a copy.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::store::paths::tmp_dir;
use crate::Result;
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn temp_object(root: &Path) -> Result<NamedTempFile> {
    fs::create_dir_all(tmp_dir(root))?; // first write in a fresh repository has no tmp dir yet
    Ok(NamedTempFile::new_in(tmp_dir(root))?) // unique name; removed automatically if dropped
}
