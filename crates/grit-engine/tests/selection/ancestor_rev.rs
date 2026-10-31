// ancestor_rev.rs — `<rev>~N` walks first parents; bare `~` means `~1`.
// Why: `grit diff <save>~1 <save>` is how you list what one save changed.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{fresh_repo, save_snapshot};
use grit_engine::refs::resolve_revision;
use std::fs;

#[test]
fn tilde_walks_first_parent_and_errors_past_root() {
    let repo = fresh_repo();
    let root = repo.path();
    fs::write(root.join("f.txt"), "1").unwrap();
    let first = save_snapshot(root, "first").unwrap();
    fs::write(root.join("f.txt"), "2").unwrap();
    let second = save_snapshot(root, "second").unwrap();
    let short = &second.hex()[..12];                         // prefix form, as `grit log` prints it
    assert_eq!(resolve_revision(root, &format!("{short}~1")).unwrap(), first);
    assert_eq!(resolve_revision(root, &format!("{short}~")).unwrap(), first); // bare ~ = ~1
    assert_eq!(resolve_revision(root, "HEAD~1").unwrap(), first);            // refs work as base
    assert_eq!(resolve_revision(root, &format!("{short}~0")).unwrap(), second);
    assert!(resolve_revision(root, "HEAD~2").is_err());      // first save has no parent
}
