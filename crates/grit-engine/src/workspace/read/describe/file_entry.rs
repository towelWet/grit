// file_entry.rs — describe a regular file, reusing the cached id when its stat still matches.
// Why: an unchanged file costs one stat instead of a full read, which is the whole point of the cache.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::index::{FileStat, Index};
use crate::objects::tree::EntryKind;
use crate::workspace::read::hash_file::hash_file;
use crate::workspace::WorkspaceEntry;
use crate::Result;
use std::fs::Metadata;
use std::path::Path;

pub fn file_entry(
    abs: &Path,
    path: &str,
    metadata: &Metadata,
    index: &Index,
    mode: u32,
) -> Result<WorkspaceEntry> {
    let stat = FileStat::from_metadata(metadata); // captured before any read, so a later write shows
    let hash = match index.lookup(path, &stat) {
        Some(cached) => cached,  // stat proves the bytes are unchanged: no read at all
        None => hash_file(abs)?, // cache miss or racy entry: read and hash the content
    };
    Ok(WorkspaceEntry {
        kind: EntryKind::File,
        hash,
        size: stat.size,
        mode,
        target: None,
        stat: Some(stat), // carried through so the next snapshot can refresh the cache
    })
}
