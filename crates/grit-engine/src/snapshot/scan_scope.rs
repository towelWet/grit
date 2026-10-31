// scan_scope.rs — scan the selected scope and record each path's entry, without writing any object yet.
// Why: classification needs only content ids, so objects are written only after a change is known to exist.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::index::Index;
use crate::objects::TreeEntry;
use crate::workspace::{scan, selected, WorkspaceEntry};
use crate::Result;
use std::collections::BTreeMap;
use super::types::SnapshotOptions;

pub fn scan_scope(
    opts: &SnapshotOptions,
    previous: &Index,
    new: &mut BTreeMap<String, TreeEntry>,
) -> Result<BTreeMap<String, WorkspaceEntry>> {
    let mut kept = BTreeMap::new(); // selected scan results, later stored and cached
    for (path, item) in scan(opts.root, opts.ignore, opts.paths, previous)? {
        if selected(&path, opts.paths) {
            new.insert(path.clone(), item.tree_entry(&path)?); // the id is known from the scan alone
            kept.insert(path, item);
        }
    }
    Ok(kept)
}
