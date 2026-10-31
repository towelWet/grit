// child_hash.rs — the stored subtree id of one top-level directory in a commit's root tree.
// Why: nested-tree tests compare directory ids between commits to prove a clean subtree was reused.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use grit_engine::objects::Commit;
use grit_engine::store::read::read_tree;
use grit_engine::Hash;
use std::path::Path;

pub fn child_hash(root: &Path, commit: &Commit, name: &str) -> Hash {
    read_tree(root, &commit.root_tree).unwrap().entries[name].hash // subtree id of one directory
}
