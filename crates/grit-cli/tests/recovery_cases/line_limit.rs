use std::path::Path;
use walkdir::WalkDir;

#[test]
fn rust_files_do_not_exceed_fifty_lines() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut violations = Vec::new();
    for scope in ["crates", "tests"] {
        for entry in WalkDir::new(root.join(scope)).into_iter().flatten() {
            if entry.path().extension().is_none_or(|value| value != "rs") {
                continue;
            }
            let source = std::fs::read_to_string(entry.path()).unwrap();
            let lines = source.lines().count();
            if lines > 50 {
                violations.push(format!("{}: {lines}", entry.path().display()));
            }
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
