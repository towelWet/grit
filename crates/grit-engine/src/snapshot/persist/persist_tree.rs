// persist_tree.rs — write the root tree of a snapshot, reusing every clean subtree from the parent.
// Why: the root is always dirty when a snapshot has changes, so this is the entry point for the recursive rebuild.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::build_dir::build_dir;
use crate::hash::Hash;
use crate::objects::TreeEntry;
use crate::snapshot::types::ObjectStats;
use crate::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub fn persist_tree(
    root: &Path,
    flat: &BTreeMap<String, TreeEntry>,
    dirty: &BTreeSet<String>,
    old_root: Option<Hash>, // parent's root tree when reusable; None rebuilds everything
    stats: &mut ObjectStats,
) -> Result<Hash> {
    build_dir(root, "", flat, dirty, old_root, stats) // "" is the repository root
}
