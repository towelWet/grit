// stall_verify.rs — verify rereads only what it has not proven; --full rereads all.
// Why: rehashing whole history took 19+ minutes; shortcut must still catch changed objects.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{run, run_ok, write};

#[test]
fn verify_trusts_only_untouched_objects() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "a.txt", "alpha");
    run_ok(root, &["snapshot", "-m", "first"]);
    assert!(!run_ok(root, &["verify"]).contains("already proven"));
    assert!(run_ok(root, &["verify"]).contains("already proven")); // second run rereads nothing
    assert!(!run_ok(root, &["verify", "--full"]).contains("already proven"));
    let object = walkdir::WalkDir::new(root.join(".grit/objects"))
        .into_iter()
        .map(|entry| entry.unwrap())
        .find(|entry| entry.file_type().is_file())
        .unwrap();
    std::fs::write(object.path(), "bitrot, different size").unwrap();
    assert!(!run(root, &["verify"]).status.success(), "changed object must be reread and caught");
}
