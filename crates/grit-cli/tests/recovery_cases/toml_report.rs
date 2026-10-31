use super::support::{run_ok, write};
use std::fs;

#[test]
fn toml_snapshot_report_and_partial_restore_work() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "a.txt", "one");
    write(root, "b.txt", "keep");
    let output = run_ok(
        root,
        &[
            "snapshot",
            "--format",
            "toml",
            "--metadata",
            "carve_run_id=42",
            "-m",
            "carve pre-run 42",
        ],
    );
    let report: toml::Value = toml::from_str(&output).unwrap();
    let id = report["snapshot_id"].as_str().unwrap();
    write(root, "a.txt", "two");
    write(root, "b.txt", "changed");
    run_ok(
        root,
        &[
            "restore",
            id,
            "--path",
            "a.txt",
            "--force",
            "--no-safety-save",
        ],
    );
    assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(), "one");
    assert_eq!(fs::read_to_string(root.join("b.txt")).unwrap(), "changed");
}
