// load.rs — the parent commit's entries, normalized so a fresh scan compares equal to unchanged files.
// Why: legacy parents name files by JSON-blob hashes; commit_entries rewrites those to raw ids.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::TreeEntry;
use crate::store::read::commit_entries;
use crate::Hash;
use crate::Result;
use std::collections::BTreeMap;
use std::path::Path;

pub(super) fn old_entries(
    root: &Path,
    parent: Option<&Hash>,
) -> Result<BTreeMap<String, TreeEntry>> {
    let Some(hash) = parent else {
        return Ok(BTreeMap::new()); // first snapshot: everything is new
    };
    Ok(commit_entries(root, hash)?.entries) // comparisons use the comparable ids only
}
