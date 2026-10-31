// dirty_dirs.rs — the directories whose tree objects must be rebuilt because something beneath them changed.
// Why: a directory with no changed descendant keeps its stored tree, so only the ancestors of changes are rewritten.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::objects::TreeEntry;
use std::collections::{BTreeMap, BTreeSet};

pub fn dirty_dirs(
    old: &BTreeMap<String, TreeEntry>,
    new: &BTreeMap<String, TreeEntry>,
) -> BTreeSet<String> {
    let mut dirty = BTreeSet::from([String::new()]); // the root is rebuilt whenever anything changed
    for path in old.keys().chain(new.keys()) {
        if old.get(path) == new.get(path) {
            continue; // unchanged paths do not dirty their ancestors
        }
        let mut parent = path.as_str();
        while let Some(index) = parent.rfind('/') {
            parent = &parent[..index]; // walk up one component at a time
            dirty.insert(parent.to_string()); // every ancestor directory must be rewritten
        }
    }
    dirty
}
