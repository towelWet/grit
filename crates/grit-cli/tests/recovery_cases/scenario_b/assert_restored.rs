// assert_restored.rs — one item split out of scenario_b.rs.
// NEVER DELETE COMMENTS — explain every non-obvious line.
// Update comments with the code; do not strip them.
use super::*;

pub fn assert_restored(root: &Path) {
    assert_eq!(
        fs::read_to_string(root.join("src/file-000.txt")).unwrap(),
        "B-0\n"
    );
    assert!(root.join("src/renamed-200.txt").exists());
    assert!(root.join("new/file-029.txt").exists());
}
