// working_content.rs — the bytes a working-tree entry has right now, for building patch text.
// Why: the scan keeps no file bytes, so a diff reads only the paths that actually changed.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::tree::EntryKind;
use crate::workspace::WorkspaceEntry;
use crate::Result;
use std::fs;
use std::path::Path;

pub fn working_content(root: &Path, path: &str, entry: &WorkspaceEntry) -> Result<Option<Vec<u8>>> {
    match entry.kind {
        EntryKind::Directory => Ok(None), // directories have no content to diff
        EntryKind::Symlink => Ok(entry.target.clone()), // a symlink's content is its target bytes
        EntryKind::File => Ok(Some(fs::read(root.join(path))?)), // read from disk; only changed paths get here
    }
}
