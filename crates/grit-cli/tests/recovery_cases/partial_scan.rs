use super::support::{run_ok, write};
use std::process::Command;

#[cfg(unix)]
#[test]
fn partial_snapshot_skips_unrelated_special_entries() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "wanted/file.txt", "protected");
    write(root, "unrelated/ordinary.txt", "ignored");
    let fifo = root.join("unrelated/live.pipe");
    let created = Command::new("/usr/bin/mkfifo")
        .arg(&fifo)
        .status()
        .unwrap()
        .success();
    assert!(created);

    let saved = run_ok(root, &["snapshot", "-m", "partial", "--path", "wanted"]);
    assert!(saved.contains("2 files")); // The selected directory and file are the whole snapshot.
}

#[test]
fn status_path_scope_ignores_files_outside_it() {
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
