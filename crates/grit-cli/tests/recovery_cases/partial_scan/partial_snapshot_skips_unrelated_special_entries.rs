// partial_snapshot_skips_unrelated_special_entries.rs — one item split out of partial_scan.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[cfg(unix)]
#[test]
pub(super) fn partial_snapshot_skips_unrelated_special_entries() {
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
