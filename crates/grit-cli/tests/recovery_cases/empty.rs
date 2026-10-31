use super::support::run_ok;

#[test]
fn empty_repository_snapshot_is_valid() {
    let fixture = tempfile::tempdir().unwrap();
    run_ok(fixture.path(), &["init"]);
    run_ok(fixture.path(), &["snapshot", "-m", "empty"]);
    assert!(run_ok(fixture.path(), &["status"]).contains("(clean)"));
    assert!(run_ok(fixture.path(), &["verify"]).contains("0 corrupt"));
}
