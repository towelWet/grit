// compare.rs — pair an old commit's entries with a new map path by path and produce one DiffEntry per change.
// Why: every diff source (commits, working tree) reduces to this one comparison, so output stays uniform.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::content::stored;
use super::model::{DiffEntry, DiffResult};
use crate::objects::TreeEntry;
use crate::store::read::CommitEntries;
use crate::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub fn compare(
    root: &Path,
    old: &CommitEntries,
    new: &BTreeMap<String, TreeEntry>,
    content: impl Fn(&str, &TreeEntry) -> Result<Option<Vec<u8>>>, // supplies new-side bytes per source
) -> Result<DiffResult> {
    let paths: BTreeSet<_> = old.entries.keys().chain(new.keys()).cloned().collect(); // sorted union of paths
    let mut entries = Vec::new();
    for path in paths {
        let (status, before, after) = match (old.entries.get(&path), new.get(&path)) {
            (Some(a), Some(b)) if a == b => continue, // identical entries are not changes
            (Some(a), Some(b)) => ("modified", Some(a), Some(b)),
            (Some(a), None) => ("deleted", Some(a), None),
            (None, Some(b)) => ("added", None, Some(b)),
            _ => continue,
        };
        let old_bytes = before.map(|e| stored(root, e, old)).transpose()?.flatten();
        let new_bytes = after.map(|e| content(&path, e)).transpose()?.flatten();
        entries.push(DiffEntry::new(
            path, status, before, after, old_bytes, new_bytes,
        ));
    }
    Ok(DiffResult::from_entries(entries))
}
