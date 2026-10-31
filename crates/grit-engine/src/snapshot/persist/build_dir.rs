// build_dir.rs — write one directory's tree object, recursing only into dirty subdirectories.
// Why: a clean directory's stored tree is still correct, so its id is reused and no child is touched.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::direct_children::direct_children;
use super::store_tree::store_tree;
use crate::hash::Hash;
use crate::objects::tree::EntryKind;
use crate::objects::{Tree, TreeEntry};
use crate::snapshot::types::ObjectStats;
use crate::store::read::read_tree;
use crate::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(super) fn build_dir(
    root: &Path,
    prefix: &str,
    flat: &BTreeMap<String, TreeEntry>,
    dirty: &BTreeSet<String>,
    old: Option<Hash>, // this directory's stored tree in the parent, if it has one
    stats: &mut ObjectStats,
) -> Result<Hash> {
    if let Some(hash) = old {
        if !dirty.contains(prefix) {
            return Ok(hash); // clean: nothing beneath it changed, so the stored tree is still exact
        }
    }
    let old_children = match old {
        Some(hash) => read_tree(root, &hash)?.entries, // only dirty directories need their old children
        None => BTreeMap::new(),
    };
    let mut entries = BTreeMap::new();
    for (path, entry) in direct_children(flat, prefix) {
        let name = path.rsplit('/').next().unwrap_or(path.as_str()).to_string(); // the child's name within this tree
        let mut child = entry.clone();
        if child.kind == EntryKind::Directory {
            let old_child = old_children
                .get(&name)
                .filter(|known| known.kind == EntryKind::Directory)
                .map(|known| known.hash); // reuse the child's old tree when it is still clean
            child.hash = build_dir(root, path, flat, dirty, old_child, stats)?; // subtree id replaces the flat id
        }
        entries.insert(name, child);
    }
    store_tree(root, &Tree::new(entries), stats) // the object id is the hash of this directory's JSON
}
