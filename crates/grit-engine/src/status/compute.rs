// compute.rs — status of the working tree against HEAD, scoped by --path when given.
// Why: a status must compare the same flat, normalized maps a snapshot would produce.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::model::StatusEntry;
use crate::ignore::IgnoreRules;
use crate::workspace::{scan, selected};
use crate::Result;
use std::path::Path;

pub fn compute_status(
    root: &Path,
    ignore: &IgnoreRules,
    selections: Option<&[String]>, // --path scope; None = whole root
) -> Result<Vec<StatusEntry>> {
    let mut old = super::old_entries::old_entries(root)?;
    old.retain(|path, _| selected(path, selections)); // unscoped files must not read as deleted
    let index = crate::index::Index::load(root); // cached ids let clean files skip the read
    let current = scan(root, ignore, selections, &index)?; // walk only the scope (home repos hit TCC dirs)
    let mut entries = super::change::entries(&old, &current)?;
    super::rename::append_path_changes(&mut entries, &old, &current)?;
    entries.sort_by(|left, right| left.path.cmp(&right.path)); // stable, path-ordered output
    Ok(entries)
}
