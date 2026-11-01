// status_path_scope_ignores_files_outside_it.rs — one item split out of partial_scan.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[test]
pub(super) fn status_path_scope_ignores_files_outside_it() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "wanted/file.txt", "v1");
    write(root, "other/file.txt", "v1");
    run_ok(root, &["snapshot", "-m", "base"]);
    write(root, "other/file.txt", "v2"); // dirty outside the scope
    let scoped = run_ok(root, &["status", "--path", "wanted"]);
    assert!(!scoped.contains("other/file.txt")); // scope hides unrelated edits
    assert!(run_ok(root, &["status"]).contains("other/file.txt")); // unscoped still sees it
}
