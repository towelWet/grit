// assert_restored.rs — one item split out of scenario_a.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn assert_restored(root: &Path) {
    assert_eq!(
        fs::read_to_string(root.join("src/file-000.txt")).unwrap(),
        "A-0\n"
    );
    assert!(root.join("src/file-200.txt").exists());
    assert!(!root.join("src/renamed-200.txt").exists());
    assert!(!root.join("new/file-000.txt").exists());
    assert!(root.join("empty/directory").is_dir());
    assert_eq!(mode(&root.join("bin/run.sh")), 0o755);
    assert_eq!(
        fs::read_link(root.join("bin/link")).unwrap(),
        Path::new("../src/file-000.txt")
    );
}
