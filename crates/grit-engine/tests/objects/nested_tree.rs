// nested_tree.rs — a second snapshot rewrites only the changed directory's tree and records only the changed path.
// Why: snapshot cost must follow change size, and history must not repeat unchanged state.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{child_hash, fresh_repo, save_snapshot};
use grit_engine::store::read::{commit_entries, read_commit};
use grit_engine::Hash;
use std::fs;

#[test]
fn clean_subtree_keeps_its_id_and_commit_lists_only_the_change() {
    let repo = fresh_repo();
    let root = repo.path();
    fs::create_dir_all(root.join("a")).unwrap();
    fs::create_dir_all(root.join("b")).unwrap();
    fs::write(root.join("a/x.txt"), "one").unwrap();
    fs::write(root.join("b/y.txt"), "keep").unwrap();
    let first = save_snapshot(root, "first").unwrap();
    fs::write(root.join("a/x.txt"), "two").unwrap(); // only directory "a" becomes dirty
    let second = save_snapshot(root, "second").unwrap();

    let before = read_commit(root, &first).unwrap();
    let after = read_commit(root, &second).unwrap();
    assert_eq!(child_hash(root, &before, "b"), child_hash(root, &after, "b")); // clean subtree reused
    assert_ne!(child_hash(root, &before, "a"), child_hash(root, &after, "a")); // dirty subtree rewritten
    assert_eq!(after.changed_paths, vec!["a/x.txt".to_string()]); // history names only the change
    let flat = commit_entries(root, &second).unwrap().entries;
    assert_eq!(flat["a/x.txt"].hash, Hash::of_blob(b"two")); // flat view matches a fresh scan
    let empty = grit_engine::objects::empty_dir_hash::empty_dir_hash().unwrap();
    assert_eq!(flat["b"].hash, empty); // flat directory entries keep the constant id scans produce
}
