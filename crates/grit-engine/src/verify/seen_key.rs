// seen_key.rs — identity of one object file as it sits on disk: path, size, mtime.
// Why: any later change to the file changes its key, so that object gets reread.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::Result;
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::DirEntry;

pub(super) fn seen_key(objects: &Path, entry: &DirEntry) -> Result<String> {
    let meta = std::fs::metadata(entry.path())?;
    let nanos = meta.modified()?.duration_since(UNIX_EPOCH).map_or(0, |t| t.as_nanos());
    let name = entry.path().strip_prefix(objects).unwrap_or(entry.path());
    Ok(format!("{} {} {nanos}", name.display(), meta.len()))
}
