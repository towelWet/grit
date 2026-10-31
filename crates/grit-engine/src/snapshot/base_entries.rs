// base_entries.rs — the starting map for a snapshot: everything the parent recorded that the scan will not re-read.
// Why: a path-scoped snapshot must keep unrelated parent entries, while a whole-root snapshot starts empty.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::types::SnapshotOptions;
use crate::objects::TreeEntry;
use crate::workspace::selected;
use std::collections::BTreeMap;

pub fn base_entries(
    opts: &SnapshotOptions,
    old: &BTreeMap<String, TreeEntry>,
) -> BTreeMap<String, TreeEntry> {
    let mut new = if opts.paths.is_some() {
        old.clone() // partial snapshot keeps every unselected entry from the parent
    } else {
        BTreeMap::new() // whole-root snapshot rebuilds everything from the scan
    };
    new.retain(|path, _| !selected(path, opts.paths)); // selected paths are re-recorded by the scan
    new
}
