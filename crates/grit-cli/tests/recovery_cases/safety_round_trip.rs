use super::safety_assertions;
use super::support::{run, run_ok, set_mode, snapshot_id, symlink, write};
use std::fs;

#[test]
fn safety_restore_preserves_every_entry_kind() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    let original = [0, 255, 10, 13, 42];
    fs::write(root.join("exact.bin"), original).unwrap();
    write(root, "deleted.txt", "restore me\n");
    write(root, "old-name.txt", "rename me\n");
    write(root, "bin/run.sh", "#!/bin/sh\nexit 0\n");
    fs::create_dir_all(root.join("empty/directory")).unwrap();
    set_mode(&root.join("bin/run.sh"), 0o755);
    symlink("exact.bin", &root.join("exact-link"));
    let baseline = snapshot_id(&run_ok(root, &["snapshot", "-m", "baseline"]));
    fs::write(root.join("exact.bin"), [9, 8, 7]).unwrap();
    fs::rename(root.join("deleted.txt"), root.join(".grit/deleted.txt")).unwrap();
    fs::rename(root.join("old-name.txt"), root.join("new-name.txt")).unwrap();
    fs::rename(root.join("empty"), root.join(".grit/removed-empty")).unwrap();
    fs::rename(root.join("exact-link"), root.join(".grit/removed-link")).unwrap();
    set_mode(&root.join("bin/run.sh"), 0o644);
    let status = run_ok(root, &["status"]);
    assert!(status.contains("D deleted.txt") && status.contains("R old-name.txt -> new-name.txt"));
    assert!(status.contains("P bin/run.sh") && status.contains("D empty/directory"));
    assert!(!run(root, &["restore", &baseline]).status.success());
    run_ok(root, &["restore", &baseline, "--force"]);
    safety_assertions::restored(root, original);
    let backups = root.join(".grit/tmp/restore-backups");
    let backup = fs::read_dir(backups)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    assert_eq!(fs::read(backup.join("exact.bin")).unwrap(), [9, 8, 7]);
    assert!(run_ok(root, &["verify"]).contains("0 corrupt"));
    run_ok(root, &["undo"]);
    safety_assertions::undone(root);
    assert!(run_ok(root, &["status"]).contains("(clean)"));
}
