// symlink_entry.rs — describe a symbolic link by its target bytes, never by what it points to.
// Why: snapshots record the link itself; following it would store a different file under the link's name.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::tree::EntryKind;
use crate::workspace::metadata::symlink_target;
use crate::workspace::WorkspaceEntry;
use crate::Result;
use std::path::Path;

pub fn symlink_entry(abs: &Path, mode: u32) -> Result<WorkspaceEntry> {
    let target = symlink_target(abs)?;
    Ok(WorkspaceEntry {
        kind: EntryKind::Symlink,
        hash: Hash::of_blob(&target), // a symlink is stored as its target bytes
        size: target.len() as u64,
        mode,
        target: Some(target),
        stat: None, // symlinks are cheap to read, so they are never cached
    })
}
