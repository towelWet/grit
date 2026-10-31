// read.rs — turn one walked directory entry into a WorkspaceEntry keyed by its repository-relative path.
// Why: the scan needs one uniform record per path whether it is a file, directory, or symlink.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::WorkspaceEntry;
use crate::index::Index;
use crate::Result;
use std::fs;
use std::path::Path;
use walkdir::DirEntry;

mod describe;
mod hash_file;
mod non_utf8;
mod vanished; // ENOENT between walk and read = skip, not abort

pub(super) fn read_entry(
    root: &Path,
    item: &DirEntry,
    index: &Index,
) -> Result<Option<(String, WorkspaceEntry)>> {
    let relative = item.path().strip_prefix(root).unwrap_or(item.path());
    if relative.as_os_str().is_empty() {
        return Ok(None); // the repository root itself is not an entry
    }
    let path = relative.to_str().ok_or_else(non_utf8::non_utf8)?.to_string();
    let described = fs::symlink_metadata(item.path()) // lstat: symlinks are described, never followed
        .map_err(Into::into)
        .and_then(|metadata| describe::describe(item.path(), &path, &metadata, index));
    match described {
        Err(error) if vanished::vanished(&error) => Ok(None), // deleted after walk → not in snapshot
        other => other.map(|entry| Some((path, entry))),
    }
}
