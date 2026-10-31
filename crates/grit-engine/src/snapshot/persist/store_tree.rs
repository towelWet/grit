// store_tree.rs — store one tree object and count whether it was new.
// Why: tree objects are part of the stats a snapshot reports, so every write must be counted or skipped.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::Tree;
use crate::snapshot::types::ObjectStats;
use crate::store::read::object_exists;
use crate::store::write::write_object;
use crate::Result;
use std::path::Path;

pub(super) fn store_tree(root: &Path, tree: &Tree, stats: &mut ObjectStats) -> Result<Hash> {
    let bytes = serde_json::to_vec(tree)?; // the JSON bytes are what the object id is computed from
    let hash = Hash::of(&bytes);
    if object_exists(root, &hash) {
        stats.reused += bytes.len() as u64; // an identical tree is already stored
    } else {
        stats.count += 1; // a new tree object is about to be written
        stats.added += bytes.len() as u64;
    }
    write_object(root, &bytes)
}
