// entry.rs — one scanned working-tree path: its content id, kind, mode, symlink target, and stat.
// Why: snapshot, status, and restore compare these fields against stored trees; file bytes are not kept in memory.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::index::FileStat;
use crate::objects::tree::EntryKind;

mod tree_entry;

#[derive(Debug, Clone)]
pub struct WorkspaceEntry {
    pub kind: EntryKind,
    pub hash: Hash,              // object id: raw-content id for files, empty-tree id for directories
    pub size: u64,               // file length in bytes; 0 for directories and symlinks
    pub mode: u32,               // full st_mode, including file-type bits
    pub target: Option<Vec<u8>>, // symlink destination bytes; None for other kinds
    pub stat: Option<FileStat>,  // present for regular files only, so the index can cache them
}
