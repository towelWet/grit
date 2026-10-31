use super::support::{run, run_ok, write};
use std::fs;

#[test]
fn stale_lock_recovers_and_live_lock_rejects_writer() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "file", "value");
    fs::write(root.join(".grit/lock"), "999999").unwrap();
    run_ok(root, &["snapshot", "-m", "stale lock"]);
    fs::write(root.join(".grit/lock"), std::process::id().to_string()).unwrap();
    assert!(!run(root, &["snapshot", "-m", "must fail"]).status.success());
}
