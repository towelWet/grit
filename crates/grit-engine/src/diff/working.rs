// working.rs — diff a saved state against the live working tree.
// Why: `grit diff` answers "what would the next snapshot change?" without writing anything.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::compare::compare;
use super::working_content::working_content;
use crate::ignore::IgnoreRules;
use crate::index::Index;
use crate::store::read::commit_entries;
use crate::workspace::scan;
use crate::{Hash, Result};
use std::collections::BTreeMap;
use std::path::Path;

pub fn diff_working_tree(
    root: &Path,
    hash: &Hash,
    ignore: &IgnoreRules,
) -> Result<super::DiffResult> {
    let old = commit_entries(root, hash)?; // comparable ids, so legacy files do not show as modified
    let index = Index::load(root); // unchanged files keep their cached ids
    let current = scan(root, ignore, None, &index)?;
    let new = current
        .iter()
        .map(|(path, entry)| Ok((path.clone(), entry.tree_entry(path)?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    compare(root, &old, &new, |path, _| match current.get(path) {
        Some(entry) => working_content(root, path, entry), // new-side bytes come from disk, for patches
        None => Ok(None),                                   // deleted paths have no new-side bytes
    })
}
