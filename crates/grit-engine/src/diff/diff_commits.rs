// diff_commits.rs — diff two saved states, each read as normalized entries.
// Why: legacy and raw commits must compare by content, not by which blob format each one used.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::compare::compare;
use super::content::stored;
use super::model::DiffResult;
use crate::store::read::commit_entries;
use crate::{Hash, Result};
use std::path::Path;

pub fn diff_commits(root: &Path, old: &Hash, new: &Hash) -> Result<DiffResult> {
    let old_entries = commit_entries(root, old)?; // flat maps plus the object ids that hold their bytes
    let new_entries = commit_entries(root, new)?;
    compare(root, &old_entries, &new_entries.entries, |_, entry| {
        stored(root, entry, &new_entries) // new side reads through its own object ids
    })
}
