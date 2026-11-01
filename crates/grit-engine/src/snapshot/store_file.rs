// store_file.rs — store one file's current bytes, noting if it changed or vanished since scan.
// Why: agents and logs write during snapshots; those races must never fail a snapshot.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::apply_raced::Raced;
use crate::store::write::write_blob;
use crate::workspace::WorkspaceEntry;
use crate::Result;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::sync::Mutex;

pub(super) fn store_file(
    root: &Path,
    path: &str,
    item: &WorkspaceEntry,
    raced: &Mutex<Vec<Raced>>,
) -> Result<()> {
    let file = match File::open(root.join(path)) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            raced.lock().unwrap().push((path.to_string(), None)); // temp file gone since scan: same as deleted
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let (stored, _) = write_blob(root, file)?; // re-read: only on a miss
    if stored != item.hash {
        // Edited after the scan: keep bytes actually stored. Old id names no object, and
        // failing here made snapshots unusable while agents or logs were writing.
        raced.lock().unwrap().push((path.to_string(), Some(stored)));
    }
    Ok(())
}
