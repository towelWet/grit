// prepare.rs — build the old and new flat entry maps that a snapshot classifies and records.
// Why: classification and tree writing both need the complete maps before anything is committed.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::base_entries::base_entries;
use super::parent_root::parent_root;
use super::scan_scope::scan_scope;
use super::types::SnapshotOptions;
use crate::index::{now_ns, Index};
use crate::objects::TreeEntry;
use crate::workspace::{validate_relative, WorkspaceEntry};
use crate::{Hash, Result};
use std::collections::BTreeMap;

pub struct Prepared {
    pub parent: Option<Hash>,
    pub old: BTreeMap<String, TreeEntry>, // parent commit's entries, normalized for comparison
    pub new: BTreeMap<String, TreeEntry>, // the state being saved
    pub kept: BTreeMap<String, WorkspaceEntry>, // scanned entries to store if this snapshot records a change
    pub index: Index, // stat cache to persist once the snapshot is committed
    pub old_root: Option<Hash>, // parent's nested root, reused for clean directories when present
}

pub fn prepare(opts: &SnapshotOptions) -> Result<Prepared> {
    for path in opts.paths.unwrap_or_default() {
        validate_relative(path)?; // reject ".." and absolute paths before touching the disk
    }
    let parent = crate::refs::get_head_hash(opts.root).ok(); // no HEAD yet => first snapshot
    let old = super::load::old_entries(opts.root, parent.as_ref())?;
    let mut new = base_entries(opts, &old);
    let scanned_ns = now_ns(); // taken before the scan: the racy-timestamp guard depends on it
    let previous = Index::load(opts.root);
    let old_root = parent_root(opts.root, parent.as_ref())?; // None unless the parent is nested
    let kept = scan_scope(opts, &previous, &mut new)?; // scan and record ids; nothing is written yet
    let index = Index::updated(previous, &kept, opts.paths, scanned_ns);
    Ok(Prepared {
        parent,
        old,
        new,
        kept,
        index,
        old_root,
    })
}
