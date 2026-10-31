use super::support::{run, run_ok, snapshot_id};

#[test]
fn five_hundred_file_round_trip_is_exact() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    super::scenario_a::populate(root);
    let a = snapshot_id(&run_ok(root, &["snapshot", "-m", "A"]));
    super::scenario_b::mutate(root);
    let status = run_ok(root, &["status"]);
    assert!(status.contains("R src/file-200.txt -> src/renamed-200.txt"));
    let report = run_ok(root, &["status", "--format", "toml"]);
    let report: toml::Value = toml::from_str(&report).unwrap();
    assert!(!report["renamed_files"].as_array().unwrap().is_empty());
    let b = snapshot_id(&run_ok(root, &["snapshot", "-m", "B"]));
    assert!(run_ok(root, &["status"]).contains("(clean)"));
    assert!(!run(root, &["restore", &a, "--no-safety-save"])
        .status
        .success());
    run_ok(root, &["restore", &a, "--force", "--no-safety-save"]);
    super::scenario_a::assert_restored(root);
    run_ok(root, &["restore", &b, "--force", "--no-safety-save"]);
    super::scenario_b::assert_restored(root);
    assert!(run_ok(root, &["verify"]).contains("0 corrupt"));
}
