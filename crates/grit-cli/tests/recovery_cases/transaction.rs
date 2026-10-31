use super::support::{run_ok, write};
use std::fs;

#[test]
fn save_finishes_with_parseable_committed_transaction() {
    let fixture = tempfile::tempdir().unwrap();
    let root = fixture.path();
    run_ok(root, &["init"]);
    write(root, "safe.txt", "protected\n");
    run_ok(root, &["snapshot", "-m", "atomic transaction"]);

    let directory = root.join(".grit/transactions");
    let paths: Vec<_> = fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(paths.len(), 1);
    let value: toml::Value = toml::from_str(&fs::read_to_string(&paths[0]).unwrap()).unwrap();
    assert_eq!(value["state"].as_str(), Some("committed"));
    assert!(fs::read_dir(directory).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".tmp")
    }));
}
