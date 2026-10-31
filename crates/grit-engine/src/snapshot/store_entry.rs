// store_entry.rs — make sure one scanned file or symlink has its object, reading the file only when it must be written.
// Why: an unchanged file already has its object, so a snapshot of an unchanged tree reads nothing.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::types::ObjectStats;
use crate::objects::tree::EntryKind;
use crate::store::read::object_exists;
use crate::store::write::write_blob;
use crate::workspace::WorkspaceEntry;
use crate::{GritError, Result};
use std::fs::File;
use std::path::Path;

pub fn store_entry(root: &Path, path: &str, item: &WorkspaceEntry) -> Result<ObjectStats> {
    let mut stats = ObjectStats::default(); // per-entry counts; the caller sums them across workers
    if item.kind == EntryKind::Directory {
        return Ok(stats); // directories are stored as the trees built from their children, never as their own objects
    }
    if object_exists(root, &item.hash) {
        stats.reused += item.size; // identical object already stored; nothing is read or written
        return Ok(stats);
    }
    stats.count += 1;
    stats.added += item.size;
    if item.kind == EntryKind::Symlink {
        write_blob(root, item.target.as_deref().unwrap_or_default())?; // target bytes are the content
    } else {
        let (stored, _) = write_blob(root, File::open(root.join(path))?)?; // re-read: only on a miss
        if stored != item.hash {
            // The file changed between the scan and this read; recording the old id would
            // point the tree at an object that does not exist, so refuse and let the user retry.
            return Err(GritError::Internal(format!("changed during snapshot: {path}")));
        }
    }
    Ok(stats)
}
