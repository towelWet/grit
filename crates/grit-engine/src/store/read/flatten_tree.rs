// flatten_tree.rs — expand a nested (per-directory) root tree into the flat path-keyed map that comparisons use.
// Why: status, diff, and restore compare paths, so the nested layout must present the same view a flat tree did.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use crate::hash::Hash;
use crate::objects::empty_dir_hash::empty_dir_hash;
use crate::objects::tree::EntryKind;
use crate::objects::TreeEntry;
use crate::store::read::read_tree;
use crate::Result;
use std::collections::BTreeMap;
use std::path::Path;

pub fn flatten_tree(root: &Path, root_tree: &Hash) -> Result<BTreeMap<String, TreeEntry>> {
    let mut flat = BTreeMap::new();
    let mut pending = vec![(String::new(), *root_tree)]; // (directory path, tree object) still to expand
    while let Some((prefix, hash)) = pending.pop() {
        for (name, entry) in read_tree(root, &hash)?.entries {
            let path = if prefix.is_empty() {
                name.clone() // entries of the root tree have no parent prefix
            } else {
                format!("{prefix}/{name}")
            };
            if entry.kind == EntryKind::Directory {
                pending.push((path.clone(), entry.hash)); // the directory's hash names its own tree object
                flat.insert(
                    path,
                    TreeEntry {
                        hash: empty_dir_hash()?, // flat view uses the constant directory id, as scans do
                        ..entry
                    },
                );
            } else {
                flat.insert(path, entry);
            }
        }
    }
    Ok(flat)
}
