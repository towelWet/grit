// collect_items.rs — walk the tree sequentially and keep every in-scope, snapshot-able entry.
// Why: the walk itself is I/O-bound and order-sensitive; the expensive per-file work is parallelized afterwards.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::{included, special};
use crate::ignore::IgnoreRules;
use crate::Result;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub(super) fn collect_items(
    root: &Path,
    ignore: &IgnoreRules,
    selections: Option<&[String]>,
) -> Result<Vec<DirEntry>> {
    let mut items = Vec::new();
    let walk = WalkDir::new(root).follow_links(false).into_iter(); // symlinks are described, not followed
    for item in walk.filter_entry(|item| included::included(root, ignore, selections, item)) {
        let item = item?; // walk errors (for example EPERM) propagate with their own path
        if !special::special(&item) {
            items.push(item); // sockets, devices, and FIFOs are dropped before any read
        }
    }
    Ok(items)
}
