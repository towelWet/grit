// toml_restore.rs — `restore --format toml` must print exactly one TOML document, dry or forced.
// Why: a human "Restored" line after the TOML would break every script that parses stdout.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::support::{run, run_ok, write};
use std::fs;

#[test]
fn restore_toml_is_one_document() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "a.txt", "one\n");
    let saved: toml::Value =
        toml::from_str(&run_ok(root, &["snapshot", "--format", "toml", "-m", "base"])).unwrap();
    let id = saved["snapshot_id"].as_str().unwrap().to_string();
    write(root, "a.txt", "two\n");

    // A plan with changes is refused without --force; the refusal is an error, but stdout still parses.
    let dry = run(root, &["restore", &id, "--format", "toml"]);
    assert!(!dry.status.success());
    let dry: toml::Value = toml::from_str(&String::from_utf8(dry.stdout).unwrap()).unwrap();
    assert_eq!(dry["restored"].as_bool(), Some(false)); // nothing ran
    assert_eq!(dry["snapshot_id"].as_str(), Some(id.as_str()));
    assert_eq!(dry["overwrite"][0].as_str(), Some("a.txt"));

    let forced: toml::Value = toml::from_str(&run_ok(
        root,
        &["restore", &id, "--format", "toml", "--force", "--no-safety-save"],
    ))
    .unwrap();
    assert_eq!(forced["restored"].as_bool(), Some(true)); // the forced run really happened
    assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(), "one\n");

    // Second run: the tree already matches, so the plan is empty and nothing runs.
    let again: toml::Value = toml::from_str(&run_ok(
        root,
        &["restore", &id, "--format", "toml", "--force", "--no-safety-save"],
    ))
    .unwrap();
    assert_eq!(again["restored"].as_bool(), Some(false));
}
