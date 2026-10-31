// directory_entry.rs — describe a directory with the shared empty-tree id.
// Why: directories carry no content of their own; their children are separate entries.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::empty_dir_hash::empty_dir_hash;
use crate::objects::tree::EntryKind;
use crate::workspace::WorkspaceEntry;
use crate::Result;

pub fn directory_entry(mode: u32) -> Result<WorkspaceEntry> {
    let hash = empty_dir_hash()?; // every directory shares this id
    Ok(WorkspaceEntry {
        kind: EntryKind::Directory,
        hash,
        size: 0, // no bytes are stored for a directory itself
        mode,
        target: None,
        stat: None, // directories are re-described on every scan; they are never cached
    })
}
