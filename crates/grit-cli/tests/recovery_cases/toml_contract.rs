// toml_contract.rs — verify, branch, and tag must answer `--format toml` with the keys scripts rely on.
// Why: the output contract (docs/output-schema.toml) promises these keys, and a silent human-text reply would break automation.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{run_ok, write};

#[test]
fn verify_branch_and_tag_emit_toml() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "a.txt", "one\n");
    run_ok(root, &["snapshot", "-m", "base"]);

    let verify: toml::Value =
        toml::from_str(&run_ok(root, &["verify", "--format", "toml"])).unwrap();
    assert_eq!(verify["ok"].as_bool(), Some(true)); // mirrors the exit code
    assert_eq!(verify["corrupt"].as_integer(), Some(0));

    let branch: toml::Value =
        toml::from_str(&run_ok(root, &["branch", "create", "topic", "--format", "toml"])).unwrap();
    assert_eq!(branch["created"].as_str(), Some("topic"));
    let listed: toml::Value =
        toml::from_str(&run_ok(root, &["branch", "--format", "toml"])).unwrap();
    assert_eq!(listed["current"].as_str(), Some("main"));
    assert!(listed["branches"].as_array().unwrap().iter().any(|b| b.as_str() == Some("topic")));

    let tag: toml::Value =
        toml::from_str(&run_ok(root, &["tag", "create", "v1", "--format", "toml"])).unwrap();
    assert_eq!(tag["name"].as_str(), Some("v1"));
    assert_eq!(tag["hash"].as_str().unwrap().len(), 64); // full id, not the short form
    let tags: toml::Value = toml::from_str(&run_ok(root, &["tag", "--format", "toml"])).unwrap();
    assert_eq!(tags["tags"][0]["name"].as_str(), Some("v1"));
}
