// complete_command_surface_stays_coherent.rs — one item split out of commands.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

#[test]
pub(super) fn complete_command_surface_stays_coherent() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "state.txt", "A\n");
    let a = snapshot_id(&run_ok(root, &["snapshot", "-m", "A"]));
    run_ok(root, &["branch", "create", "feature"]);
    write(root, "state.txt", "B\n");
    let b = snapshot_id(&run_ok(root, &["snapshot", "-m", "B"]));
    run_ok(root, &["tag", "create", "baseline", &a]);
    assert!(run_ok(root, &["branch"]).contains("feature"));
    assert!(run_ok(root, &["tag"]).contains("baseline"));
    parse_toml(&run_ok(root, &["show", &a, "--format", "toml"]));
    parse_toml(&run_ok(root, &["diff", &a, &b, "--format", "toml"]));
    parse_toml(&run_ok(root, &["log", "--format", "toml"]));
    run_ok(root, &["checkout", "feature", "--force"]);
    assert_eq!(
        std::fs::read_to_string(root.join("state.txt")).unwrap(),
        "A\n"
    );
    assert!(run_ok(root, &["status"]).contains("(clean)"));
    run_ok(root, &["checkout", "main", "--force"]);
    run_ok(root, &["restore", &a, "--force"]);
    assert_eq!(
        std::fs::read_to_string(root.join("state.txt")).unwrap(),
        "A\n"
    );
    run_ok(root, &["undo"]);
    assert_eq!(
        std::fs::read_to_string(root.join("state.txt")).unwrap(),
        "B\n"
    );
    assert!(run_ok(root, &["status"]).contains("(clean)"));
    assert!(run_ok(root, &["verify"]).contains("0 corrupt"));
    assert!(run_ok(root, &["gc"]).contains("GC:"));
    assert!(run_ok(root, &["doctor"]).contains("Branch: main"));
}
