// no_op_snapshot.rs — a snapshot with nothing to record must create no commit and say so.
// Why: empty commits pollute history, and `log` would overstate how many real saves happened.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{run_ok, write};

#[test]
fn clean_snapshot_creates_no_commit() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "a.txt", "one\n");
    let first = run_ok(root, &["snapshot", "-m", "first"]);
    assert!(first.starts_with("Saved as "), "{first}"); // a real change still reports the id
    let report = run_ok(root, &["snapshot", "--format", "toml", "-m", "again"]);
    let report: toml::Value = toml::from_str(&report).unwrap();
    assert_eq!(report["saved"].as_bool(), Some(false)); // no commit was written
    assert!(report.get("id").is_none()); // so there is no new id to report
    assert_eq!(report["files"].as_integer(), Some(0));
    assert_eq!(run_ok(root, &["snapshot", "-m", "again"]).trim(), "Nothing to save (clean)");
    let log = run_ok(root, &["log", "--format", "toml"]);
    assert_eq!(log.matches("message = ").count(), 1); // only the first save reached history
}
